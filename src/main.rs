extern crate gematria_rs;
use std::io::{self, Read};
use clap::{Parser, Subcommand, ValueEnum};
use gematria_rs::{GematriaBuilder, GematriaMethod};

/// Simple program to calculate a gematric value from hebrew words or phrases
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,

    /// The gematria calculation method.
    #[clap(short, long, value_enum)]
    method: Option<GematriaMethods>,
    
    /// Enable caching for repeated calculations.
    #[clap(short = 'c', long)]
    enable_cache: bool,

    /// Preserve vowels in the words.
    #[clap(short = 'p', long)]
    preserve_vowels: bool,
    
    /// Enable verbose outputs.
    #[clap(short = 'v', long)]
    verbose: bool,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Calculate the gematria value of a single word or phrase.
    Calculate {
        /// The word or phrase for which to calculate the gematria value.
        text: String,
    },
    /// Search for words with a gematria value equal to that of a specific word.
    SearchMatch {
        /// The word to compare against.
        word: String,
        /// The text to search within.
        text: Option<String>,
    },
    /// Groups words with matching gematria values.
    GroupWords {
        /// The text to search within.
        text: Option<String>,
    },
}

#[derive(ValueEnum, Clone, Debug)]
enum GematriaMethods {
    MisparHechrechi,
    MisparGadol,
    MisparKatan,
    OtiyotBeMilui,
}

fn main() {
    let cli = Cli::parse();
    let mut builder = GematriaBuilder::new()
        .with_cache(cli.enable_cache)
        .with_vowels(cli.preserve_vowels);

    if let Some(m) = cli.method {
        builder = builder.with_method(GematriaMethod::from(m));
    }

    let gematria_context = builder.init_gematria();
    match cli.command {
        Commands::Calculate { text } => {
            
            let result = gematria_context.calculate_value(&text);

            if cli.verbose {
                println!("Gematria value for '{}': {}", text, result.value());
            } else {
                println!("{}", result.value());
            }
        },
        Commands::SearchMatch { word, text } => {
            // Logic for searching within the text to find words matching the gematria value of 'word'
            let text_to_search = match text {
                Some(t) => t,
                None => {
                    let mut buffer = String::new();
                    io::stdin().read_to_string(&mut buffer).expect("Failed to read from stdin");
                    buffer
                },
            };

            let matching_words = gematria_context.search_matching_words(&word, &text_to_search);
        
            for matching_word in matching_words {
                println!("{}", matching_word);
            }
        },
        Commands::GroupWords { text } => {
            let text_to_search = match text {
                Some(t) => t,
                None => {
                    let mut buffer = String::new();
                    io::stdin().read_to_string(&mut buffer).expect("Failed to read from stdin");
                    buffer
                },
            };
            match gematria_context.group_words_by_gematria(&text_to_search) {
                Ok(groups) => {
                    for (value, words) in groups {
                        if cli.verbose {
                            println!("Gematria value {:4}: {}", value, words.join(", "));
                        } else {
                            println!("{:4} -> {}", value, words.join(", "));
                        }
                    }
                },
                Err(e) => eprintln!("Error reading file: {}", e),
            }
        }
    }
}

impl From<GematriaMethods> for GematriaMethod {
    fn from(method: GematriaMethods) -> Self {
        match method {
            GematriaMethods::MisparHechrechi => GematriaMethod::MisparHechrechi,
            GematriaMethods::MisparGadol => GematriaMethod::MisparGadol,
            GematriaMethods::MisparKatan => GematriaMethod::MisparKatan,
            GematriaMethods::OtiyotBeMilui => GematriaMethod::OtiyotBeMilui,
        }
    }
}
