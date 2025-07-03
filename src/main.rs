use anyhow::Result;
use clap::{Arg, Command};

mod config;
mod writer;

use config::Config;
use writer::TweetWriter;

fn main() -> Result<()> {
    let matches = Command::new("tw")
        .about("Twitter風つぶやきをMarkdownファイルに記録するCLIツール")
        .version("0.1.0")
        .arg(Arg::new("text").help("記録するテキスト").index(1))
        .get_matches();

    let text = if let Some(text) = matches.get_one::<String>("text") {
        text.clone()
    } else {
        println!("つぶやきを入力してください:");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        input.trim().to_string()
    };

    if text.is_empty() {
        println!("テキストが入力されていません。");
        return Ok(());
    }

    let config = Config::load_or_create()?;
    let writer = TweetWriter::new(config);

    writer.write_tweet(&text)?;
    println!("つぶやきを記録しました: {}", text);

    Ok(())
}
