mod matrix;

use std::time::Duration;

use color_eyre::{Result, eyre::Context};
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode},
    text::Text,
};

use matrix::Matrix;

fn main() -> Result<()> {
    color_eyre::install()?; // augment errors / panics with easy to read messages

    let terminal = ratatui::init();

    let app_result = run(terminal).context("app loop failed");
    ratatui::restore();

    app_result
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    let size = terminal.size().context("failed to fetch terminal size")?;
    let mut matrix = Matrix::new(size.width as usize, size.height as usize);

    loop {
        terminal.draw(|frame| draw_matrix(frame, matrix.string()))?;

        if should_quit()? {
            break;
        }

        matrix.add_line("haha");
    }

    Ok(())
}

fn draw_matrix(frame: &mut Frame, matrix: String) {
    let tx = Text::raw(&matrix);
    frame.render_widget(tx, frame.area());
}

fn should_quit() -> Result<bool> {
    if event::poll(Duration::from_millis(250)).context("event poll failed")? {
        if let Event::Key(key) = event::read().context("event read failed")? {
            return Ok(KeyCode::Char('q') == key.code);
        }
    }

    Ok(false)
}
