pub struct StaticText {
    pub chars: Vec<char>,
}

impl StaticText {
    pub fn new(text: &str) -> Self {
        StaticText {
            chars: text.chars().collect(),
        }
    }
}
