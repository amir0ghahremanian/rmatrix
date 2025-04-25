mod matrix;

use std::time::Duration;

use color_eyre::{
    Result,
    eyre::{Context, Ok},
};
use rand::{random_ratio, rng, seq::IndexedRandom};
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode},
    layout::Rect,
    style::{Color, Style},
    text::Line,
    widgets::{Borders, Clear, ListState, Padding},
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
                matrix.dim();

                let menu_block = ratatui::widgets::Block::default()
                    .borders(Borders::ALL)
                    .padding(Padding::uniform(1))
                    .title("Menu");
                let menu_items = ["Settings", "Return"];
                let menu_list = ratatui::widgets::List::new(menu_items)
                    .block(menu_block)
                    .highlight_style(Style::new().fg(Color::Black).bg(Color::White))
                    .style(Style::new().fg(Color::White));
                let mut menu_state = ListState::default();

                let area = Rect::new(4, 2, 12, 6);
                loop {
                    match listen()? {
                        KeyEvent::Up => {
                            menu_state.select_previous();
                        }
                        KeyEvent::Down => {
                            menu_state.select_next();
                        }
                        KeyEvent::Enter => {
                            match menu_state.selected() {
                                Some(0) => {}
                                Some(1) => break,
                                _ => {}
                            };
                        }
                        KeyEvent::Quit => break,
                        _ => {}
                    }

                    terminal.draw(|frame| {
                        frame.render_widget(matrix.text(), frame.area());
                        frame.render_widget(Clear, area);
                        frame.render_stateful_widget(&menu_list, area, &mut menu_state);
                    })?;
                }

                matrix.normal();
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

enum KeyEvent {
    Quit,
    Menu,
    None,
    Up,
    Down,
    Enter,
}

fn listen() -> Result<KeyEvent> {
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
