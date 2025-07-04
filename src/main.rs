use anyhow::Result;
use clap::{Arg, Command};

mod config;
mod writer;

use config::Config;
use writer::TweetWriter;

fn main() -> Result<()> {
    let matches = Command::new("tw")
        .about("CLI tool for recording Twitter-style tweets to Markdown files")
        .version("0.1.0")
        .arg(Arg::new("text").help("Text to record").index(1))
        .get_matches();

    // Load or create config first
    let config = Config::load_or_create()?;

    let text = if let Some(text) = matches.get_one::<String>("text") {
        text.clone()
    } else {
        println!("Please enter your tweet:");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        input.trim().to_string()
    };

    if text.is_empty() {
        println!("No text entered.");
        return Ok(());
    }

    let writer = TweetWriter::new(config);

    writer.write_tweet(&text)?;
    println!("Tweet recorded: {}", text);

    Ok(())
}
