pub struct SectionManager {
    snippet: String,
    sections: Vec<Section>,
    current_section: usize,
}

impl SectionManager {
    pub fn new(content: &str) -> Self {
        SectionManager {
            snippet: "Hello".to_string(),
            sections: Vec::new(),
            current_section: 0,
        }
    }
}

#[cfg(test)]
mod tests {}
