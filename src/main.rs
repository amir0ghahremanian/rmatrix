mod matrix;
mod menu;

use std::time::Duration;

use color_eyre::{
    Result,
    eyre::{Context, Ok},
};
use rand::{random_ratio, rng, seq::IndexedRandom};
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode},
    text::Line,
};

use matrix::Matrix;
use menu::menu;

fn main() -> Result<()> {
    // augment errors / panics with easy to read messages
    color_eyre::install()?;

    let terminal = ratatui::init();

    let app_result = run(terminal).context("app loop failed");
    ratatui::restore();

    app_result
}

const PROP_COME: u32 = 5;
const PROP_STAY: u32 = 85;

fn matrix_transform(before: &str) -> String {
    before
        .chars()
        .map(|x| {
            let prop = match x.is_alphabetic() {
                true => PROP_STAY,
                false => PROP_COME,
            };

            let sample = ['A', 'B', 'C', 'D', 'a', 'b', 'c', 'd'];

            match random_ratio(prop, 100) {
                true => *sample.choose(&mut rng()).unwrap(),
                false => ' ',
            }
        })
        .collect()
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    let size = terminal.size().context("failed to fetch terminal size")?;

    let mut matrix = Matrix::new(size.height as usize);
    let mut generator: String = vec![' '; size.width as usize].into_iter().collect();

    loop {
        terminal.draw(|frame| draw(frame, matrix.text()))?;

        match listen()? {
            KeyEvent::Menu => {
                menu(&mut terminal, &mut matrix)?;
            }
            KeyEvent::Quit => break,
            _ => {}
        };

        generator = matrix_transform(&generator);

        matrix.push_line_on_top(Line::raw(generator.clone()));
    }

    Ok(())
}

fn draw<W>(frame: &mut Frame, widget: W)
where
    W: ratatui::widgets::Widget,
{
    frame.render_widget(widget, frame.area());
}

pub enum KeyEvent {
    Quit,
    Menu,
    None,
    Up,
    Down,
    Enter,
}

pub fn listen() -> Result<KeyEvent> {
    if event::poll(Duration::from_millis(250)).context("event poll failed")? {
        match event::read().context("event read failed")? {
            Event::Key(key) => match key.code {
                KeyCode::Char('q') => return Ok(KeyEvent::Quit),
                KeyCode::Char('m') => return Ok(KeyEvent::Menu),
                KeyCode::Up => return Ok(KeyEvent::Up),
                KeyCode::Down => return Ok(KeyEvent::Down),
                KeyCode::Enter => return Ok(KeyEvent::Enter),
                _ => return Ok(KeyEvent::None),
            },
            _ => return Ok(KeyEvent::None),
        };
    }

    Ok(KeyEvent::None)
}
