use clap::{Parser, Subcommand};
use rand::Rng;

mod config;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Quotes,
    Authors,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Quotes => {
                for quote in config::QUOTES {
                    let author = quote.0;
                    let message = quote.1;
                    println!("{message} - {author}")
                }
            }
            Commands::Authors => {
                let mut authors: Vec<&str> = vec![];

                for quote in config::QUOTES {
                    let author = quote.0;
                    if !authors.contains(&author) {
                        authors.push(author);
                    }
                }

                for author in authors {
                    println!("{author}");
                }
            }
        }

        return;
    }

    let mut thread_rng = rand::rng();
    let index = thread_rng.random_range(0..config::QUOTES.len());
    let quote = config::QUOTES[index];
    let author = quote.0;
    let message = quote.1;
    println!("{message} - {author}")
}
