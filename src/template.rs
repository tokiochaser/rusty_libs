use crate::word_type::{WordType, match_word_type};

#[derive(Debug)]
pub enum TemplatePiece {
    Text(String),
    Placeholder(WordType),
}

#[derive(Debug)]
pub struct Template {
    pub name: String,
    pub parts: Vec<TemplatePiece>,
}

impl Template {
    pub fn new(name: &str, parts: Vec<TemplatePiece>) -> Self
    {
        Self {
            name: name.to_string(),
            parts,
        }
    }

    pub fn fill(&self) -> String {
        let mut result = String::new();
        for part in &self.parts {
            match part {
                TemplatePiece::Text(text) => result.push_str(&text),
                TemplatePiece::Placeholder(word_type) => {
                    let word = match_word_type(*word_type);
                    result.push_str(&word);
                }
            }
        }
        result
    }
}
