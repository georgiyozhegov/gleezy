#[derive(Clone, Debug)]
pub struct Span {
    row: u64,
    column: u64,
}

impl Span {
    pub fn new(row: u64, column: u64) -> Self {
        Self { row, column }
    }

    pub fn row(&self) -> u64 {
        self.row
    }

    pub fn column(&self) -> u64 {
        self.column
    }

    pub fn update(&mut self, c: char) {
        match c {
            '\n' => {
                self.row += 1;
                self.column = 1;
            }
            _ => {
                self.column += 1;
            }
        }
    }
}
