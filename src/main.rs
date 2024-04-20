use anyhow::Result;
use chrono::{Local, Timelike};
use crossterm::{
    event::{self, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::*, widgets::Paragraph};
use std::io::stdout;

mod text;

fn main() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    loop {
        terminal.draw(|frame| render(frame).expect("failed to render"))?;

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

fn render(frame: &mut Frame) -> Result<()> {
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
        "{:02}:{:02}:{:02}.{:05}",
        now.hour(),
        now.minute(),
        now.second(),
        now.nanosecond() / 10000
    ));
    let value: Vec<Line<'static>> = value.into_iter().map(|s| s.green().into()).collect();

    frame.render_widget(
        Paragraph::new(value).alignment(Alignment::Center),
        layout[1],
    );

    Ok(())
}
