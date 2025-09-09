# Rustacean Mad Libs 🦀

![LISENCE](image.png) 

A command-line Mad Libs game built for Rustaceans, by a Rustacean. This fun word game prompts you for various parts of speech to create a hilarious and often nonsensical story about a technical interview.

##  Overview

Mad Libs is a phrasal template word game where one player prompts others for a list of words to substitute for blanks in a story, often with hilarious results. This implementation is a Rust version of that classic game, featuring a tech-themed story built with memory safety and zero-cost abstractions in mind!

##  Features

- **Strongly-Typed Nonsense:** All user input is mapped to a word type enum.

- **Built with Rust:** Because why make an easy project easy for myself?


##  Using the program

### Prerequisites

You just need the Rust toolchain installed on your machine.

``` unix
curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh
```

Windows installer: [Install Rust - Rust Programming Language](https://www.rust-lang.org/tools/install)

### Playing the game

1. **Clone the repository:**

2. **Run the game with Cargo**
	cargo run --release

## How It Works

The game will prompt you for different types of words (adjectives, nouns, verbs, etc.) one by one. After you've entered all the required words, it will compile your inputs into an, often silly, story about a technical interview and print it to the console.

## Project Structure

The code is organized into three main modules:

- `main.rs`: The application entry point and story template.

- `template.rs`: Defines the `Template` and `TemplatePiece` logic for building and filling the story.

- `word_type.rs`: Handles the `WordType` enum and user input prompting.

## Contributions

Contributions welcome! Whether you have an idea for a template, a design improvement, or a code refinement, feel free to open an Issue or submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE]([https://license/](https://github.com/tokiochaser/rusty_libs/blob/main/LICENSE.md)) file for details.

## Acknowledgments

- Inspired by the classic Mad Libs game.

- Built with the elegant Rust programming language.

- Thanks to [The Rust Programming Language Forum](https://users.rust-lang.org/) for invaluable resources and community support :)
