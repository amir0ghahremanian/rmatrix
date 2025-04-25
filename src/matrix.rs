use ratatui::{
    style::{Color, Style, Stylize},
    text::{Line, Text},
};

pub struct Matrix<'a> {
    inner: Text<'a>,
    max_length: usize,
}

#[allow(dead_code)]
impl<'a> Matrix<'a> {
    pub fn new(max_length: usize) -> Self {
        let style = Style::new().fg(Color::Green);

        Self {
            inner: Text {
                style,
                ..Default::default()
            },
            max_length,
        }
    }

    pub fn push_line_on_top<L>(&mut self, line: L)
    where
        L: Into<Line<'a>>,
    {
        if self.inner.lines.len() == self.max_length {
            self.inner.lines.pop();
        }

        self.inner.push_line(line);
        self.inner.lines.rotate_right(1);
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

    pub fn dim(&mut self) {
        self.inner.style = self.inner.style.fg(Color::Rgb(40, 66, 40));
    }

    pub fn normal(&mut self) {
        self.inner.style = self.inner.style.fg(Color::Green);
    }
}
