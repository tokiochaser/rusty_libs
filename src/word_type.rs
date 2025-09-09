//! word_type.rs for our mad libs game
use std::io::{self, Write};

// word types are contained in an enum
#[derive(Debug, Copy, Clone)]
pub enum WordType {
    Adjective,
    Adverb,
    PresentVerb,
    PastVerb,
    Noun,
    PluralNoun,
    PlaceName,
    PersonName,
    PartOfBody,
    Exclamation,
}

/// a helper function for prompting for and recieving input
fn prompt_user (prompt: &str, buffer: &mut String, error: &str) {
    print!("{}", prompt);

    io::stdout()
        .flush()
        .expect("Failed to flush output");
    
    io::stdin()
        .read_line(buffer)
        .expect(error);
}

/// matching the WordType enumerations to an input prompt
pub fn match_word_type(word_type: WordType) -> String {
    let mut word = String::new();
    
    match word_type {
        WordType::Adjective => {
            prompt_user("Enter an adjective: ", &mut word, "Failed to get adjective");
        },
        WordType::Adverb => {
            prompt_user("Enter an adverb: ", &mut word, "Failed to get adverb");
        },
        WordType::PresentVerb => {
            prompt_user("Enter a present tense verb: ", &mut word, "Failed to get present tense verb");
        },
        WordType::PastVerb => {
            prompt_user("Enter a past tense verb: ", &mut word, "Failed to get past tense verb");
        },
        WordType::Noun => {
            prompt_user("Enter a noun: ", &mut word, "Failed to get noun");
        },
        WordType::PluralNoun => {
            prompt_user("Enter a plural noun: ", &mut word, "Failed to get plural noun");
        },
        WordType::PlaceName => {
            prompt_user("Enter a place name: ", &mut word, "Failed to get place name");
        },
        WordType::PersonName => {
            prompt_user("Enter a person's name: ", &mut word, "Failed to get person's name");
        },
        WordType::PartOfBody => {
            prompt_user("Enter a part of the body: ", &mut word, "Failed to get part of body");
        },
        WordType::Exclamation => {
            prompt_user("Enter an exclamation: ", &mut word, "Failed to get exclamation");
        }
    }
    
    word.trim().to_string()
}