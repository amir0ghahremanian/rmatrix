use color_eyre::eyre::Result;
use ratatui::{
    DefaultTerminal,
    layout::Rect,
    style::{Color, Style},
    widgets::{Borders, Clear, ListState, Padding},
};

use crate::{KeyEvent, listen, matrix::Matrix};

pub fn menu(terminal: &mut DefaultTerminal, matrix: &mut Matrix) -> Result<()> {
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

    Ok(())
}
