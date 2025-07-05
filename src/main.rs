use anyhow::Result;
use clap::{Arg, Command};
use std::io::{self, Write};

mod config;
mod writer;

use config::Config;
use writer::TweetWriter;

fn main() -> Result<()> {
    let matches = Command::new("tm")
        .about("CLI tool for recording Twitter-style tweets to Markdown files")
        .version("0.1.0")
        .arg(Arg::new("text").help("Text to record").index(1))
        .get_matches();

    // Load or create config first
    let config = Config::load_or_create()?;

    if let Some(text) = matches.get_one::<String>("text") {
        // Single tweet mode
        let writer = TweetWriter::new(config);
        writer.write_tweet(text)?;
        println!("Tweet recorded: {}", text);
    } else {
        // Interactive mode
        interactive_mode(config)?;
    }

    Ok(())
}

fn interactive_mode(config: Config) -> Result<()> {
    let writer = TweetWriter::new(config);

    println!(
        "Interactive mode started. Type your tweets and press Enter. Type 'quit' or 'exit' to stop."
    );
    println!("Press Ctrl+C to exit at any time.");
    println!();

    loop {
        print!("> ");
        io::stdout().flush()?;

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let text = input.trim();

                if text.is_empty() {
                    continue;
                }

                if text == "quit" || text == "exit" {
                    println!("Goodbye!");
                    break;
                }

                match writer.write_tweet(text) {
                    Ok(_) => {
                        // 前の行（プロンプト + 入力内容）を上書き
                        print!("\x1B[1A\r\x1B[K");
                        // 入力内容をタイムスタンプ付きで表示
                        let formatted = writer.format_tweet_display(text);
                        println!("{}", formatted);
                    }
                    Err(e) => {
                        print!("\x1B[1A\r\x1B[K");
                        eprintln!("Error recording tweet: {}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                break;
            }
        }
    }

    Ok(())
}
