
#[derive(Clone, Copy)]
pub struct Cursor {
    pub x: usize,
    pub y: usize,
}

impl std::fmt::Display for Cursor {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{{x: {}, y: {}}}", self.x, self.y)
    }
}

impl Cursor {
    fn line(self, text: String) -> String {
        match text.split('\n').nth(self.y + 1) {
            Some(line) => line.to_string(),
            None => "".to_string(),
        }
    }

    pub fn home(self) -> Cursor {
        Cursor { x: 0, y: self.y }
    }

    pub fn end(self, text: String) -> Cursor {
        Cursor {
            x: self.line(text).chars().count(),
            y: self.y,
        }
    }

    pub fn back(self) -> Cursor {
        Cursor {
            x: if self.x == 0 { 0 } else { self.x - 1 },
            y: self.y,
        }
    }

    pub fn forward(self, text: String) -> Cursor {
        let len = self.line(text).chars().count();
        Cursor {
            x: if self.x < len { self.x + 1 } else { len },
            y: self.y,
        }
    }
}
