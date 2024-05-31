use crate::section::Section;

pub struct SnippetManager {
    title: String,
    sections: Vec<Section>,
    current_section: usize,
}

impl SnippetManager {
    pub fn new(title: &str, content: &str) -> Self {
        SnippetManager {
            title: title.to_owned(),
            sections: Vec::new(),
            current_section: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SnippetManager;

    #[test]
    fn correct_title_set() {
        let manager = SnippetManager::new("My title", "Hello this is just text");
        assert_eq!(manager.title, "My title");
    }
}
