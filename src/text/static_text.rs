#[derive(Debug, PartialEq)]
pub struct StaticText {
    pub chars: Vec<char>,
}

impl StaticText {
    pub fn new(chars: Vec<char>) -> Self {
        StaticText { chars }
    }
}
