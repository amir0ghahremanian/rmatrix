use ratatui::text::{Line, Text};

pub struct Matrix<'a>(Text<'a>);

impl<'a> Matrix<'a> {
    pub fn new() -> Self {
        let mut s = Self(Text::raw(""));

        s.0.lines.clear();

        s
    }

    pub fn write_line<L>(&mut self, line: L)
    where
        L: Into<Line<'a>>,
    {
        self.0.push_line(line);
    }

    pub fn text(&self) -> &Text {
        &self.0
    }
}
