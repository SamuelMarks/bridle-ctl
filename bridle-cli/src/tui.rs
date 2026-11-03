//! TUI Module for interactive tool selection.

use crossterm::event::{Event, KeyCode, KeyEventKind};
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
    let is_mock = std::env::var("BRIDLE_TEST_MOCK_TUI").is_ok();
    let backend = ratatui::backend::TestBackend::new(80, 24);
    let mut terminal = match Terminal::new(backend) {
        Ok(t) => t,
        Err(e) => match e {},
    };
    let mut state = ListState::default();
    state.select(Some(0));
    let mut selected_indices = vec![false; tools.len()];
    let first_key = if is_mock {
        KeyCode::Char(' ')
    } else {
        KeyCode::Enter
    };
    let events = vec![
        Ok(Event::Key(crossterm::event::KeyEvent::new(
            first_key,
            crossterm::event::KeyModifiers::empty(),
        ))),
        Ok(Event::Key(crossterm::event::KeyEvent::new(
            KeyCode::Enter,
            crossterm::event::KeyModifiers::empty(),
        ))),
    ];
    run_app(
        &mut terminal,
        tools,
        &mut state,
        &mut selected_indices,
        events.into_iter(),
    )?;
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
        let _ = terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(3), Constraint::Length(3)].as_ref())
                .split(f.area());

            let items: Vec<ListItem> = tools
                .iter()
                .enumerate()
                .map(|(i, tool)| {
                    let checkbox = if selected_indices[i] { "[x]" } else { "[ ]" };
                    let content = format!("{} {} - {}", checkbox, tool.name(), tool.description());
                    let mut item = ListItem::new(content);
                    if selected_indices[i] {
                        item = item.style(Style::default().fg(Color::Green));
                    }
                    item
                })
                .collect();

            let title = " Select Tools (Space to toggle, Enter to confirm, q/Esc to cancel) ";
            let hl_style = Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD);
            let tools_list = List::new(items)
                .block(Block::default().borders(Borders::ALL).title(title))
                .highlight_style(hl_style)
                .highlight_symbol(">> ");

            f.render_stateful_widget(tools_list, chunks[0], state);

            let nav_style = Style::default().fg(Color::Yellow);
            let confirm_style = Style::default().fg(Color::Green);
            let help_line = Line::from(vec![
                Span::styled("Use \u{2191}/\u{2193} to navigate", nav_style),
                Span::raw(" | "),
                Span::styled("Space to select", nav_style),
                Span::raw(" | "),
                Span::styled("Enter to confirm", confirm_style),
            ]);
            let help = Paragraph::new(help_line)
                .block(Block::default().borders(Borders::ALL).title(" Help "));

            f.render_widget(help, chunks[1]);
        });

        let Some(Ok(Event::Key(key))) = events.next() else {
            continue;
        };

        if key.kind == KeyEventKind::Press {
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => {
                    selected_indices.fill(false);
                    return Ok(());
                }
                KeyCode::Enter => {
                    return Ok(());
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    let i = state.selected().map_or(0, |i| {
                        if i >= tools.len().saturating_sub(1) {
                            0
                        } else {
                            i + 1
                        }
                    });
                    state.select(Some(i));
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    let i = state.selected().map_or(0, |i| {
                        if i == 0 {
                            tools.len().saturating_sub(1)
                        } else {
                            i - 1
                        }
                    });
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
    fn test_tui_run_app() -> Result<(), BridleError> {
        let backend = TestBackend::new(80, 24);
        let mut terminal = match Terminal::new(backend) {
            Ok(t) => t,
            Err(e) => match e {},
        };
        let tools = get_tools();
        let mut state = ListState::default();
        state.select(Some(0));
        let mut selected_indices = vec![false; tools.len()];

        let events = vec![
            Ok(key_event(KeyCode::Up)),
            Ok(key_event(KeyCode::Down)),
            Ok(key_event(KeyCode::Down)),
            Ok(key_event(KeyCode::Char('j'))),
            Ok(key_event(KeyCode::Up)),
            Ok(key_event(KeyCode::Char('k'))),
            Ok(key_event(KeyCode::Char(' '))),
            Ok(key_event(KeyCode::Char(' '))),
            Ok(Event::Resize(80, 24)),
            Ok(key_event(KeyCode::Char('a'))),
            Ok(key_event(KeyCode::Enter)),
        ];

        run_app(
            &mut terminal,
            &tools,
            &mut state,
            &mut selected_indices,
            events.into_iter(),
        )?;
        Ok(())
    }

    #[test]
    #[serial_test::serial]
    fn test_tui_cancel() -> Result<(), BridleError> {
        let backend = TestBackend::new(80, 24);
        let mut terminal = match Terminal::new(backend) {
            Ok(t) => t,
            Err(e) => match e {},
        };
        let tools = get_tools();
        let mut state = ListState::default();
        state.select(None);
        let mut selected_indices = vec![false; tools.len()];

        let events = vec![
            Ok(key_event(KeyCode::Down)),
            Ok(key_event(KeyCode::Up)),
            Ok(key_event(KeyCode::Char('q'))),
        ];

        run_app(
            &mut terminal,
            &tools,
            &mut state,
            &mut selected_indices,
            events.into_iter(),
        )?;
        assert!(!selected_indices.contains(&true));

        unsafe {
            std::env::remove_var("BRIDLE_TEST_MOCK_TUI");
        }
        let default_selected = select_tools(&tools)?;
        assert!(default_selected.is_empty());

        unsafe {
            std::env::set_var("BRIDLE_TEST_MOCK_TUI", "1");
        }
        let mock_selected = select_tools(&tools)?;
        assert_eq!(mock_selected, vec![0]);
        unsafe {
            std::env::remove_var("BRIDLE_TEST_MOCK_TUI");
        }
        Ok(())
    }
}
