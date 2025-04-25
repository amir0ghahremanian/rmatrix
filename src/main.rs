mod matrix;

use std::time::Duration;

use color_eyre::{Result, eyre::Context};
use rand::{random_ratio, rng, seq::IndexedRandom, thread_rng};
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

const PROP_COME: u32 = 10;
const PROP_STAY: u32 = 75;

fn matrix_transform(before: &str) -> String {
    before
        .chars()
        .map(|x| {
            let mut prop = PROP_COME;

            if x.is_alphabetic() {
                prop = PROP_STAY;
            }

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

        if should_quit()? {
            break;
        }

        generator = matrix_transform(&generator);

        matrix.push_line_on_top(ratatui::text::Line::raw(generator.clone()));
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
