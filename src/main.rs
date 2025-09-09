//! main.rs for our mad libs game

pub mod word_type;
pub mod template;

use crate::template::{TemplatePiece, Template};
use crate::word_type::WordType;



fn main() {
    // creating a new madlibs template
    let template = Template::new (
        "The Tech Interview",
        vec![
            TemplatePiece::Text("My technical interview started okay...\n I was ".to_string()),
            TemplatePiece::Placeholder(WordType::Adverb),
            TemplatePiece::Text(" typing away on my ".to_string()),
            TemplatePiece::Placeholder(WordType::Noun),
            TemplatePiece::Text(".\n Unfortunately, my second problem was to make a(n) ".to_string()),
            TemplatePiece::Placeholder(WordType::Noun),
            TemplatePiece::Text(" completely from scratch!\n ".to_string()),
            TemplatePiece::Placeholder(WordType::PersonName),
            TemplatePiece::Text(", my interviewer, was pretty tough. They even called my work ".to_string()),
            TemplatePiece::Placeholder(WordType::Adjective),
            TemplatePiece::Text("!\n Luckily, I ".to_string()),
            TemplatePiece::Placeholder(WordType::Adverb),
            TemplatePiece::Text(" ".to_string()),
            TemplatePiece::Placeholder(WordType::PastVerb),
            TemplatePiece::Text(" my position as a professional software dev.\n ".to_string()),
            TemplatePiece::Placeholder(WordType::Exclamation),
            TemplatePiece::Text("!\n I finally ".to_string()),
            TemplatePiece::Placeholder(WordType::PastVerb),
            TemplatePiece::Text(" my interview and now I can ".to_string()),
            TemplatePiece::Placeholder(WordType::PresentVerb),
            TemplatePiece::Text(" and go (to) ".to_string()),
            TemplatePiece::Placeholder(WordType::PlaceName),
            TemplatePiece::Text(".".to_string()),
        ],
    );

    // printing an introduction
    println!("Welcome to Mad Libs: {}", template.name);

    // filling in the template
    let mad_libs = template.fill();

    // displaying the mad libs
    println!("\n=============================================\n");
    println!("{}", mad_libs);
}
