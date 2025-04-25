mod matrix;

use std::time::Duration;

use color_eyre::{Result, eyre::Context};
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode},
};

use matrix::Matrix;

fn main() -> Result<()> {
    // augment errors / panics with easy to read messages
    color_eyre::install()?;

    let terminal = ratatui::init();

    let app_result = run(terminal).context("app loop failed");
    ratatui::restore();

    app_result
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    let size = terminal.size().context("failed to fetch terminal size")?;
    let mut matrix = Matrix::new(size.height as usize);

    loop {
        terminal.draw(|frame| draw(frame, matrix.text()))?;

        if should_quit()? {
            break;
        }

        matrix.push_line(ratatui::text::Line::raw(matrix.len().to_string()));
    }

    Ok(())
}

fn draw<W>(frame: &mut Frame, widget: W)
where
    W: ratatui::widgets::Widget,
{
    frame.render_widget(widget, frame.area());
}

fn should_quit() -> Result<bool> {
    if event::poll(Duration::from_millis(250)).context("event poll failed")? {
        if let Event::Key(key) = event::read().context("event read failed")? {
            return Ok(KeyCode::Char('q') == key.code);
        }
    }

    Ok(false)
}
