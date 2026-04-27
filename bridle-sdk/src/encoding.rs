//! Handles reading and writing text files while preserving line endings and text encoding.

use std::borrow::Cow;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use encoding_rs::Encoding;

use crate::BridleError;

/// Analyzes a string slice to detect the predominant line ending sequence.
///
/// Returns `"\r\n"` if CRLF is detected, otherwise returns `"\n"`.
pub fn detect_line_ending(text: &str) -> &'static str {
    if text.contains("\r\n") { "\r\n" } else { "\n" }
}

/// A structure holding the decoded text, its original encoding, and line ending.
#[derive(Debug, PartialEq, Eq)]
pub struct TextDocument<'a> {
    /// The decoded UTF-8 text.
    pub text: Cow<'a, str>,
    /// The detected encoding of the original document.
    pub encoding: &'static Encoding,
    /// Whether the original document had a Byte Order Mark (BOM).
    pub has_bom: bool,
    /// The detected primary line ending of the original document.
    pub line_ending: &'static str,
}

impl<'a> TextDocument<'a> {
    /// Normalizes the line endings of the text to standard LF (`\n`).
    pub fn normalize_line_endings(&mut self) {
        if self.text.contains("\r\n") {
            self.text = Cow::Owned(self.text.replace("\r\n", "\n"));
        }
    }

    /// Converts the normalized text back to the original line endings.
    pub fn restore_line_endings(&self) -> Cow<'_, str> {
        if self.line_ending == "\r\n" && !self.text.contains("\r\n") {
            Cow::Owned(self.text.replace('\n', "\r\n"))
        } else {
            Cow::Borrowed(&self.text)
        }
    }
}

/// Reads a file from disk, automatically detecting and decoding its character set.
pub fn read_file_with_encoding<P: AsRef<Path>>(
    path: P,
) -> Result<TextDocument<'static>, BridleError> {
    let mut file = File::open(path)?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;

    // Better auto-detect using BOMs.
    let (encoding, has_bom, bytes_without_bom) = if bytes.starts_with(b"\xEF\xBB\xBF") {
        (encoding_rs::UTF_8, true, &bytes[3..])
    } else if bytes.starts_with(b"\xFF\xFE") {
        (encoding_rs::UTF_16LE, true, &bytes[2..])
    } else if bytes.starts_with(b"\xFE\xFF") {
        (encoding_rs::UTF_16BE, true, &bytes[2..])
    } else {
        (encoding_rs::UTF_8, false, bytes.as_slice()) // Default to UTF-8
    };

    let (decoded_text, _, _) = encoding.decode(bytes_without_bom);
    let line_ending = detect_line_ending(&decoded_text);

    Ok(TextDocument {
        text: Cow::Owned(decoded_text.into_owned()),
        encoding,
        has_bom,
        line_ending,
    })
}

/// Writes text to a file, re-encoding it to the specified encoding and preserving BOMs.
pub fn write_file_with_encoding<P: AsRef<Path>>(
    path: P,
    document: &TextDocument<'_>,
) -> Result<(), BridleError> {
    let mut file = File::create(path)?;

    let text_to_encode = document.restore_line_endings();

    // Write BOM if necessary
    if document.has_bom {
        if document.encoding == encoding_rs::UTF_8 {
            file.write_all(b"\xEF\xBB\xBF")?;
        } else if document.encoding == encoding_rs::UTF_16LE {
            file.write_all(b"\xFF\xFE")?;
        } else if document.encoding == encoding_rs::UTF_16BE {
            file.write_all(b"\xFE\xFF")?;
        }
    }

    if document.encoding == encoding_rs::UTF_16LE {
        let utf16: Vec<u8> = text_to_encode
            .encode_utf16()
            .flat_map(|c| c.to_le_bytes())
            .collect();
        file.write_all(&utf16)?;
    } else if document.encoding == encoding_rs::UTF_16BE {
        let utf16: Vec<u8> = text_to_encode
            .encode_utf16()
            .flat_map(|c| c.to_be_bytes())
            .collect();
        file.write_all(&utf16)?;
    } else {
        let (encoded_bytes, _, _) = document.encoding.encode(&text_to_encode);
        file.write_all(&encoded_bytes)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use tempfile::NamedTempFile;

    #[test]
    fn test_read_utf8_bom() -> Result<(), BridleError> {
        let mut temp_file = NamedTempFile::new().map_err(BridleError::Io)?;
        temp_file
            .write_all(b"\xEF\xBB\xBFhi")
            .map_err(BridleError::Io)?;
        let doc = read_file_with_encoding(temp_file.path())?;
        assert!(doc.has_bom);
        assert_eq!(doc.encoding, encoding_rs::UTF_8);
        Ok(())
    }

    #[test]
    fn test_read_utf16be_bom() -> Result<(), BridleError> {
        let mut temp_file = NamedTempFile::new().map_err(BridleError::Io)?;
        temp_file
            .write_all(b"\xFE\xFF\x00h\x00i")
            .map_err(BridleError::Io)?;
        let doc = read_file_with_encoding(temp_file.path())?;
        assert!(doc.has_bom);
        assert_eq!(doc.encoding, encoding_rs::UTF_16BE);
        Ok(())
    }

    #[test]
    fn test_write_utf8_bom() -> Result<(), BridleError> {
        let doc = TextDocument {
            text: Cow::Borrowed("hi"),
            encoding: encoding_rs::UTF_8,
            has_bom: true,
            line_ending: "\n",
        };
        let out_file = NamedTempFile::new().map_err(BridleError::Io)?;
        write_file_with_encoding(out_file.path(), &doc)?;
        let mut raw_bytes = Vec::new();
        File::open(out_file.path())
            .map_err(BridleError::Io)?
            .read_to_end(&mut raw_bytes)
            .map_err(BridleError::Io)?;
        assert_eq!(raw_bytes, b"\xEF\xBB\xBFhi");
        Ok(())
    }

    #[test]
    fn test_detect_line_ending() {
        assert_eq!(detect_line_ending("hello\nworld"), "\n");
        assert_eq!(detect_line_ending("hello\r\nworld"), "\r\n");
        assert_eq!(detect_line_ending("single line"), "\n");
    }

    #[test]
    fn test_normalize_and_restore_line_endings() {
        let mut doc = TextDocument {
            text: Cow::Borrowed("line1\r\nline2"),
            encoding: encoding_rs::UTF_8,
            has_bom: false,
            line_ending: "\r\n",
        };

        doc.normalize_line_endings();
        assert_eq!(doc.text, "line1\nline2");

        let restored = doc.restore_line_endings();
        assert_eq!(restored, "line1\r\nline2");
    }

    #[test]
    fn test_read_write_utf8_crlf() -> Result<(), BridleError> {
        let mut temp_file = NamedTempFile::new().map_err(BridleError::Io)?;
        temp_file
            .write_all(b"test\r\nfile")
            .map_err(BridleError::Io)?;

        let doc = read_file_with_encoding(temp_file.path())?;
        assert_eq!(doc.text, "test\r\nfile");
        assert_eq!(doc.line_ending, "\r\n");
        assert_eq!(doc.encoding, encoding_rs::UTF_8);

        let out_file = NamedTempFile::new().map_err(BridleError::Io)?;
        write_file_with_encoding(out_file.path(), &doc)?;

        let mut read_back = String::new();
        File::open(out_file.path())
            .map_err(BridleError::Io)?
            .read_to_string(&mut read_back)
            .map_err(BridleError::Io)?;
        assert_eq!(read_back, "test\r\nfile");

        Ok(())
    }

    #[test]
    fn test_read_write_utf16le_bom() -> Result<(), BridleError> {
        let mut temp_file = NamedTempFile::new().map_err(BridleError::Io)?;
        // UTF-16LE BOM + "hi" in utf16le
        temp_file
            .write_all(b"\xFF\xFEh\x00i\x00")
            .map_err(BridleError::Io)?;

        let doc = read_file_with_encoding(temp_file.path())?;
        assert_eq!(doc.text, "hi");
        assert!(doc.has_bom);
        assert_eq!(doc.encoding, encoding_rs::UTF_16LE);

        let out_file = NamedTempFile::new().map_err(BridleError::Io)?;
        write_file_with_encoding(out_file.path(), &doc)?;

        let mut raw_bytes = Vec::new();
        File::open(out_file.path())
            .map_err(BridleError::Io)?
            .read_to_end(&mut raw_bytes)
            .map_err(BridleError::Io)?;
        assert_eq!(raw_bytes, b"\xFF\xFEh\x00i\x00");

        Ok(())
    }

    #[test]
    fn test_write_utf16be_bom() -> Result<(), BridleError> {
        let doc = TextDocument {
            text: Cow::Borrowed("hi"),
            encoding: encoding_rs::UTF_16BE,
            has_bom: true,
            line_ending: "\n",
        };

        let out_file = NamedTempFile::new().map_err(BridleError::Io)?;
        write_file_with_encoding(out_file.path(), &doc)?;

        let mut raw_bytes = Vec::new();
        File::open(out_file.path())
            .map_err(BridleError::Io)?
            .read_to_end(&mut raw_bytes)
            .map_err(BridleError::Io)?;
        // UTF-16BE BOM + "hi" in utf16be
        assert_eq!(raw_bytes, b"\xFE\xFF\x00h\x00i");

        Ok(())
    }

    #[test]
    fn test_restore_line_endings_no_op() {
        let doc = TextDocument {
            text: Cow::Borrowed("line1\nline2"),
            encoding: encoding_rs::UTF_8,
            has_bom: false,
            line_ending: "\n",
        };
        let restored = doc.restore_line_endings();
        assert_eq!(restored, "line1\nline2");
        assert!(matches!(restored, Cow::Borrowed(_)));
    }
}
