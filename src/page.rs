pub mod menu;

use ratatui::DefaultTerminal;

pub trait Page<S, R> {
    fn enter(&mut self);

    fn render(&self, terminal: &mut DefaultTerminal) -> PageState<S, R>;

    fn exit(&mut self);
}

pub enum PageState<S, R> {
    Stay(S),
    Return(R),
}

pub fn enter_page<S, R>(terminal: &mut DefaultTerminal, page: &mut impl Page<S, R>) -> Result<()> {
    page.enter();

    let r = loop {
        match page.render(terminal) {
            PageState::Stay(_) => {}
            PageState::Return(r) => break r,
        }
    };

    page.exit();
}
