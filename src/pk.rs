use std::io;
use std::process::Command;

use color_eyre::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};

enum Mode {
    Menu,
    Input,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    enable_raw_mode()?;
    execute!(io::stdout(), EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;

    let menu = vec![
        "Install package",
        "Update packages",
        "Upgrade packages",
        "Update && Upgrade",
        "Exit",
    ];

    let mut selected = 0;
    let mut mode = Mode::Menu;
    let mut input = String::new();

    loop {
        terminal.draw(|f| {
            let size = f.size();

            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Percentage(30),
                    Constraint::Percentage(40),
                    Constraint::Percentage(30),
                ])
                .split(size);

            let middle = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(20),
                    Constraint::Percentage(60),
                    Constraint::Percentage(20),
                ])
                .split(layout[1]);

            match mode {
                Mode::Menu => {
                    let items: Vec<ListItem> = menu
                        .iter()
                        .enumerate()
                        .map(|(i, m)| {
                            let style = if i == selected {
                                Style::default()
                                    .fg(Color::Black)
                                    .bg(Color::Green)
                                    .add_modifier(Modifier::BOLD)
                            } else {
                                Style::default().fg(Color::White)
                            };
                            ListItem::new(*m).style(style)
                        })
                        .collect();

                    let list = List::new(items)
                        .block(Block::default().title("PKMG").borders(Borders::ALL));

                    f.render_widget(list, middle[1]);
                }

                Mode::Input => {
                    let boxw = Paragraph::new(input.as_str())
                        .block(Block::default().title("Enter package").borders(Borders::ALL))
                        .style(Style::default().fg(Color::Yellow))
                        .alignment(Alignment::Center);

                    f.render_widget(boxw, middle[1]);
                }
            }

            let footer = Paragraph::new("Enter: select | q: exit | type in input mode")
                .style(Style::default().fg(Color::DarkGray))
                .alignment(Alignment::Center);

            f.render_widget(footer, layout[2]);
        })?;

        if let Event::Key(key) = event::read()? {
            match mode {
                Mode::Menu => match key.code {
                    KeyCode::Up => {
                        if selected > 0 {
                            selected -= 1;
                        }
                    }

                    KeyCode::Down => {
                        if selected < menu.len() - 1 {
                            selected += 1;
                        }
                    }

                    KeyCode::Enter => {
                        match selected {
                            0 => mode = Mode::Input,
                            1 => run_cmd("pkg update")?,
                            2 => run_cmd("pkg upgrade")?,
                            3 => run_cmd("pkg update && pkg upgrade")?,
                            4 => {
                                cleanup()?;
                                return Ok(());
                            }
                            _ => {}
                        }
                    }

                    KeyCode::Char('q') => {
                        cleanup()?;
                        return Ok(());
                    }

                    _ => {}
                },

                Mode::Input => match key.code {
                    KeyCode::Char(c) => input.push(c),
                    KeyCode::Backspace => {
                        input.pop();
                    }
                    KeyCode::Enter => {
                        let cmd = format!("pkg install {}", input.trim());
                        run_cmd(&cmd)?;
                        input.clear();
                        mode = Mode::Menu;
                    }
                    KeyCode::Esc => {
                        input.clear();
                        mode = Mode::Menu;
                    }
                    _ => {}
                },
            }
        }
    }
}

fn run_cmd(cmd: &str) -> Result<()> {
    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;

    println!("\n==> {}", cmd);
    Command::new("sh").arg("-c").arg(cmd).status()?;

    println!("\nPress ENTER...");
    let mut s = String::new();
    let _ = io::stdin().read_line(&mut s);

    execute!(io::stdout(), EnterAlternateScreen)?;
    enable_raw_mode()?;

    Ok(())
}

fn cleanup() -> Result<()> {
    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;
    Ok(())
}
