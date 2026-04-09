# Add New Tools

## Purpose
`ADD_NEW_TOOLS.md` describes how to add compiled FFI tools into the `bridle-sdk` engine.

## Process
1. Build a new tool as a C-compatible shared library.
2. Expose an `audit()` and `fix()` extern "C" block.
3. Bind it in `bridle-sdk/src/ffi.rs`.
4. Run `cargo tarpaulin` to guarantee 100% test coverage for the FFI binding layer.
