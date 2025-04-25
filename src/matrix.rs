use ratatui::text::{Line, Text};

pub struct Matrix<'a> {
    inner: Text<'a>,
    max_length: usize,
}

impl<'a> Matrix<'a> {
    pub fn new(max_length: usize) -> Self {
        Self {
            inner: Text::default(),
            max_length,
        }
    }

    pub fn push_line<L>(&mut self, line: L)
    where
        L: Into<Line<'a>>,
    {
        if self.inner.lines.len() == self.max_length {
            self.inner.lines.rotate_left(1);
            self.inner.lines.pop();
        }

        self.inner.push_line(line);
    }

    pub fn text(&self) -> &Text {
        &self.inner
    }

    pub fn len(&self) -> usize {
        self.inner.lines.len()
    }
}
