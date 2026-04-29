#![allow(unused_variables, unused_imports)]

mod generator;
mod validator;

use clap::{Parser, Subcommand};
use generator::{generate_passphrase, generate_pin, generate_random};
use validator::{calculate_entropy, check_common_patterns, validate_strength};

/// A password generator CLI.
#[derive(Parser)]
#[command(name = "passgen", version, about = "Generate and validate passwords")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a random password.
    Random {
        #[arg(short, long, default_value_t = 16)]
        length: usize,
        #[arg(short, long)]
        symbols: bool,
    },
    /// Generate a passphrase from random words.
    Passphrase {
        #[arg(short, long, default_value_t = 4)]
        words: usize,
        #[arg(short, long, default_value_t = '-')]
        separator: char,
    },
    /// Generate a numeric PIN.
    Pin {
        #[arg(short, long, default_value_t = 6)]
        length: usize,
    },
    /// Check the strength of a password.
    Validate {
        password: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Random { length, symbols } => {
            let pwd = generate_random(length, symbols);
            let entropy = calculate_entropy(&pwd);
            println!("Password: {}", pwd);
            println!("Entropy:  {:.1} bits", entropy);
        }
        Commands::Passphrase { words, separator } => {
            let phrase = generate_passphrase(words, separator);
            println!("Passphrase: {}", phrase);
        }
        Commands::Pin { length } => {
            let pin = generate_pin(length);
            println!("PIN: {}", pin);
        }
        Commands::Validate { password } => {
            let strength = validate_strength(&password);
            println!("Strength: {}", strength);
            if check_common_patterns(&password) {
                println!("Warning: password matches a common pattern!");
            }
        }
    }
}