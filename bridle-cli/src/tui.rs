//! TUI Module for interactive tool selection.

use std::io;

use crossterm::{
    event::{Event, KeyCode, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};

use crate::tools::CodeTool;
use bridle_sdk::BridleError;

/// Launches the interactive TUI to select tools.
pub fn select_tools(tools: &[Box<dyn CodeTool>]) -> Result<Vec<usize>, BridleError> {
    enable_raw_mode().map_err(BridleError::Io)?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen).map_err(BridleError::Io)?;
    let backend = ratatui::backend::CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).map_err(BridleError::Io)?;

    let mut state = ListState::default();
    state.select(Some(0));
    let mut selected_indices = vec![false; tools.len()];

    let event_iter = std::iter::from_fn(|| Some(crossterm::event::read().map_err(BridleError::Io)));
    let res = run_app(
        &mut terminal,
        tools,
        &mut state,
        &mut selected_indices,
        event_iter,
    );

    disable_raw_mode().map_err(BridleError::Io)?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen).map_err(BridleError::Io)?;
    terminal.show_cursor().map_err(BridleError::Io)?;

    res?;

    let selected: Vec<usize> = selected_indices
        .into_iter()
        .enumerate()
        .filter(|(_, is_selected)| *is_selected)
        .map(|(i, _)| i)
        .collect();

    Ok(selected)
}

/// Runs the main event loop for the TUI.
fn run_app<B: Backend, I>(
    terminal: &mut Terminal<B>,
    tools: &[Box<dyn CodeTool>],
    state: &mut ListState,
    selected_indices: &mut [bool],
    mut events: I,
) -> Result<(), BridleError>
where
    I: Iterator<Item = Result<Event, BridleError>>,
{
    loop {
        terminal
            .draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Min(3), Constraint::Length(3)].as_ref())
                    .split(f.area());

                let items: Vec<ListItem> = tools
                    .iter()
                    .enumerate()
                    .map(|(i, tool)| {
                        let checkbox = if selected_indices[i] { "[x]" } else { "[ ]" };
                        let content =
                            format!("{} {} - {}", checkbox, tool.name(), tool.description());
                        let mut item = ListItem::new(content);
                        if selected_indices[i] {
                            item = item.style(Style::default().fg(Color::Green));
                        }
                        item
                    })
                    .collect();

                let tools_list = List::new(items)
                    .block(Block::default().borders(Borders::ALL).title(
                        " Select Tools (Space to toggle, Enter to confirm, q/Esc to cancel) ",
                    ))
                    .highlight_style(
                        Style::default()
                            .bg(Color::DarkGray)
                            .add_modifier(Modifier::BOLD),
                    )
                    .highlight_symbol(">> ");

                f.render_stateful_widget(tools_list, chunks[0], state);

                let help = Paragraph::new(Line::from(vec![
                    Span::styled(
                        "Use \u{2191}/\u{2193} to navigate",
                        Style::default().fg(Color::Yellow),
                    ),
                    Span::raw(" | "),
                    Span::styled("Space to select", Style::default().fg(Color::Yellow)),
                    Span::raw(" | "),
                    Span::styled("Enter to confirm", Style::default().fg(Color::Green)),
                ]))
                .block(Block::default().borders(Borders::ALL).title(" Help "));

                f.render_widget(help, chunks[1]);
            })
            .map_err(|e| BridleError::Io(std::io::Error::other(e.to_string())))?;

        let Ok(Event::Key(key)) =
            events
                .next()
                .unwrap_or(Err(BridleError::Io(std::io::Error::other(
                    "no more events",
                ))))
        else {
            continue;
        };

        if key.kind == KeyEventKind::Press {
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => {
                    // clear selection to cancel
                    selected_indices.fill(false);
                    return Ok(());
                }
                KeyCode::Enter => {
                    return Ok(());
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    let i = match state.selected() {
                        Some(i) => {
                            if i >= tools.len().saturating_sub(1) {
                                0
                            } else {
                                i + 1
                            }
                        }
                        None => 0,
                    };
                    state.select(Some(i));
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    let i = match state.selected() {
                        Some(i) => {
                            if i == 0 {
                                tools.len().saturating_sub(1)
                            } else {
                                i - 1
                            }
                        }
                        None => 0,
                    };
                    state.select(Some(i));
                }
                KeyCode::Char(' ') => {
                    if let Some(i) = state.selected() {
                        selected_indices[i] = !selected_indices[i];
                    }
                }
                _ => {}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tools::registry::get_tools;
    use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};
    use ratatui::backend::TestBackend;

    fn key_event(code: KeyCode) -> Event {
        Event::Key(KeyEvent {
            code,
            modifiers: KeyModifiers::empty(),
            kind: KeyEventKind::Press,
            state: KeyEventState::empty(),
        })
    }

    #[test]
    fn test_tui_run_app() -> Result<(), Box<dyn std::error::Error>> {
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend)?;
        let tools = get_tools();
        let mut state = ListState::default();
        state.select(Some(0));
        let mut selected_indices = vec![false; tools.len()];

        let events = vec![
            Ok(key_event(KeyCode::Down)),
            Ok(key_event(KeyCode::Char('j'))),
            Ok(key_event(KeyCode::Up)),
            Ok(key_event(KeyCode::Char('k'))),
            Ok(key_event(KeyCode::Char(' '))), // toggle
            Ok(key_event(KeyCode::Char(' '))), // toggle off
            Ok(Event::Resize(80, 24)),         // unhandled event type
            Ok(key_event(KeyCode::Char('a'))), // unhandled key
            Ok(key_event(KeyCode::Enter)),
        ];

        let res = run_app(
            &mut terminal,
            &tools,
            &mut state,
            &mut selected_indices,
            events.into_iter(),
        );

        assert!(res.is_ok());
        Ok(())
    }

    #[test]
    fn test_tui_cancel() -> Result<(), Box<dyn std::error::Error>> {
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend)?;
        let tools = get_tools();
        let mut state = ListState::default();
        state.select(None); // hit None state
        let mut selected_indices = vec![false; tools.len()];

        let events = vec![
            Ok(key_event(KeyCode::Down)), // None -> 0
            Ok(key_event(KeyCode::Up)),   // 0 -> max
            Ok(key_event(KeyCode::Char('q'))),
        ];

        let res = run_app(
            &mut terminal,
            &tools,
            &mut state,
            &mut selected_indices,
            events.into_iter(),
        );

        assert!(res.is_ok());
        assert!(!selected_indices.contains(&true));
        Ok(())
    }
}
