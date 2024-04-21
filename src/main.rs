use anyhow::{bail, Result};
use chrono::{Local, Timelike};
use crossterm::{
    event::{self, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::*, widgets::Paragraph};
use std::{io::stdout, str::FromStr};

use clap::Parser;

mod text;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Specifies the font color of the clock
    #[arg(short, long)]
    color: String,
}
fn start_ui(color: Color) -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    loop {
        terminal.draw(|frame| render(color, frame).expect("failed to render"))?;

        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    break;
                }
            }
        }
    }

    terminal.clear()?;
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
fn main() -> Result<()> {
    let args = Args::parse();

    let color = match Color::from_str(&args.color) {
        Ok(color) => color,
        Err(_) => bail!("Invalid color '{}'", args.color),
    };

    start_ui(color)
}

fn render(color: Color, frame: &mut Frame) -> Result<()> {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Fill(2),
            Constraint::Length(5),
            Constraint::Fill(2),
        ])
        .split(frame.size());

    let now = Local::now();
    let value = text::generate_lines(format!(
        "{:02}:{:02}:{:02}",
        now.hour(),
        now.minute(),
        now.second(),
    ));

    let value: Vec<Line<'static>> = value.into_iter().map(|s| s.fg(color).into()).collect();

    frame.render_widget(
        Paragraph::new(value).alignment(Alignment::Center),
        layout[1],
    );

    Ok(())
}
