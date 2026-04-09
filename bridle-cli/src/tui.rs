//! TUI Module for interactive tool selection.

use std::io;

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};

use crate::error::CliError;
use crate::tools::CodeTool;

/// Launches the interactive TUI to select tools.
#[cfg(not(tarpaulin_include))]
pub fn select_tools(tools: &[Box<dyn CodeTool>]) -> Result<Vec<usize>, CliError> {
    enable_raw_mode().map_err(CliError::Io)?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen).map_err(CliError::Io)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).map_err(CliError::Io)?;

    let mut state = ListState::default();
    state.select(Some(0));
    let mut selected_indices = vec![false; tools.len()];

    let res = run_app(&mut terminal, tools, &mut state, &mut selected_indices);

    disable_raw_mode().map_err(CliError::Io)?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen).map_err(CliError::Io)?;
    terminal.show_cursor().map_err(CliError::Io)?;

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
#[cfg(not(tarpaulin_include))]
fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    tools: &[Box<dyn CodeTool>],
    state: &mut ListState,
    selected_indices: &mut [bool],
) -> Result<(), CliError> {
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
            .map_err(CliError::Io)?;

        let Ok(Event::Key(key)) = event::read().map_err(CliError::Io) else {
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
