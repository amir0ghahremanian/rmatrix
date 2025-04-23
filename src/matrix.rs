pub struct Matrix {
    buf: String,
    curr: usize,
    w: usize,
}

impl Matrix {
    pub fn new(w: usize, h: usize) -> Self {
        Self {
            buf: String::with_capacity(h*(w+1)),
            curr: 0,
            w,
        }
    }

    pub fn add_line(&mut self, line: &str) {
        if self.curr != self.buf.len() {
            let here = self.curr * self.w;
            let next = here + self.w;
            let mut line = self.buf[here..next];
            self.buf[self.curr] = line.chars().collect();
            self.curr += 1;
        } else {
            self.buf.rotate_left(1);
            self.buf[self.curr - 1] = line.chars().collect();
        }
    }

    pub fn str(&self) -> &str {
        let mut s = String::new();

        for l in self.buf.iter() {
            s.push_str(String::from_iter(l.clone()).as_str());
            s.push('\n');
        }

        s
    }
}
