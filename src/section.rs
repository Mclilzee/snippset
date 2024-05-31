pub struct Section {
    position: (u16, u16),
    text: Vec<char>,
}

impl Section {
    pub fn new(column: u16, row: u16) -> Self {
        Section {
            position: (column, row),
            text: Vec::new(),
        }
    }
}
