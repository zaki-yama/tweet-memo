use anyhow::{anyhow, Context, Result};
use chrono::Local;
use std::fs;
use std::path::PathBuf;

use crate::config::Config;

pub struct TweetWriter {
    config: Config,
}

impl TweetWriter {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn write_tweet(&self, text: &str) -> Result<()> {
        let file_path = self.get_target_file_path()?;

        if !file_path.exists() {
            return Err(anyhow!(
                "Target file does not exist: {}",
                file_path.display()
            ));
        }

        let content = fs::read_to_string(&file_path).with_context(|| {
            format!("Failed to read file: {}", file_path.display())
        })?;

        let new_content = self.insert_tweet_into_content(&content, text)?;

        fs::write(&file_path, new_content).with_context(|| {
            format!("Failed to write file: {}", file_path.display())
        })?;

        Ok(())
    }

    fn get_target_file_path(&self) -> Result<PathBuf> {
        let expanded_dir = self.config.expand_path(&self.config.target_directory);
        let dir_path = PathBuf::from(expanded_dir);

        if !dir_path.exists() {
            return Err(anyhow!(
                "Target directory does not exist: {}",
                dir_path.display()
            ));
        }

        let now = Local::now();
        let filename = self
            .config
            .filename_format
            .replace("YYYY", &now.format("%Y").to_string())
            .replace("MM", &now.format("%m").to_string())
            .replace("DD", &now.format("%d").to_string());

        Ok(dir_path.join(filename))
    }

    fn insert_tweet_into_content(&self, content: &str, text: &str) -> Result<String> {
        let lines: Vec<&str> = content.lines().collect();
        let mut result_lines = Vec::new();
        let mut section_found = false;
        let mut insert_index = None;

        for (i, line) in lines.iter().enumerate() {
            result_lines.push(line.to_string());

            if line.trim() == self.config.target_section.trim() {
                section_found = true;
                insert_index = Some(self.find_section_end(&lines, i));
            }
        }

        if !section_found {
            return Err(anyhow!(
                "Target section '{}' not found",
                self.config.target_section
            ));
        }

        let insert_pos = insert_index.unwrap();
        let formatted_tweet = self.format_tweet(text);

        result_lines.insert(insert_pos, formatted_tweet);

        Ok(result_lines.join("\n"))
    }

    fn find_section_end(&self, lines: &[&str], section_start: usize) -> usize {
        let section_level = self.get_heading_level(&self.config.target_section);

        for i in (section_start + 1)..lines.len() {
            let line = lines[i].trim();

            if line.starts_with('#') {
                let current_level = self.get_heading_level(line);
                if current_level <= section_level {
                    return i;
                }
            }
        }

        lines.len()
    }

    fn get_heading_level(&self, line: &str) -> usize {
        let trimmed = line.trim();
        let mut count = 0;
        for char in trimmed.chars() {
            if char == '#' {
                count += 1;
            } else {
                break;
            }
        }
        count
    }

    fn format_tweet(&self, text: &str) -> String {
        let now = Local::now();
        let timestamp = now.format("%H:%M:%S").to_string();

        format!(
            "- {}",
            self.config
                .entry_format
                .replace("HH:mm:ss", &timestamp)
                .replace("{text}", text)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Datelike, Local};
    use tempfile::TempDir;

    fn create_test_config() -> Config {
        Config {
            target_directory: "/tmp".to_string(),
            filename_format: "test.md".to_string(),
            entry_format: "[HH:mm:ss] {text}".to_string(),
            target_section: "### Tweets".to_string(),
        }
    }

    #[test]
    fn test_tweet_writer_new() {
        let config = create_test_config();
        let writer = TweetWriter::new(config.clone());
        assert_eq!(writer.config.target_directory, config.target_directory);
    }

    #[test]
    fn test_get_heading_level() {
        let config = create_test_config();
        let writer = TweetWriter::new(config);

        assert_eq!(writer.get_heading_level("# Heading 1"), 1);
        assert_eq!(writer.get_heading_level("## Heading 2"), 2);
        assert_eq!(writer.get_heading_level("### Heading 3"), 3);
        assert_eq!(writer.get_heading_level("#### Heading 4"), 4);
        assert_eq!(writer.get_heading_level("Not a heading"), 0);
    }

    #[test]
    fn test_format_tweet() {
        let config = create_test_config();
        let writer = TweetWriter::new(config);
        let text = "Test tweet";
        let formatted = writer.format_tweet(text);

        assert!(formatted.starts_with("- ["));
        assert!(formatted.contains("] Test tweet"));
    }

    #[test]
    fn test_find_section_end() {
        let config = create_test_config();
        let writer = TweetWriter::new(config);

        let lines = vec![
            "# Main Title",
            "Some content",
            "### Tweets",
            "- Tweet 1",
            "- Tweet 2",
            "## Next Section",
            "More content",
        ];

        let end_pos = writer.find_section_end(&lines, 2); // Index of "### Tweets"
        assert_eq!(end_pos, 5); // Should stop at "## Next Section"
    }

    #[test]
    fn test_insert_tweet_into_content() {
        let config = create_test_config();
        let writer = TweetWriter::new(config);

        let content = r#"# Daily Notes

## Tasks
- Do something

### Tweets

### Notes
Some notes here"#;

        let result = writer.insert_tweet_into_content(content, "Test tweet").unwrap();
        assert!(result.contains("### Tweets"));
        assert!(result.contains("Test tweet"));
    }

    #[test]
    fn test_insert_tweet_section_not_found() {
        let mut config = create_test_config();
        config.target_section = "### NonExistent".to_string();
        let writer = TweetWriter::new(config);

        let content = r#"# Daily Notes

## Tasks
- Do something"#;

        let result = writer.insert_tweet_into_content(content, "Test tweet");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }

    #[test]
    fn test_filename_format_replacement() {
        let temp_dir = TempDir::new().unwrap();
        let mut config = create_test_config();
        config.target_directory = temp_dir.path().to_string_lossy().to_string();
        config.filename_format = "YYYY-MM-DD.md".to_string();
        
        let writer = TweetWriter::new(config);
        let file_path = writer.get_target_file_path().unwrap();
        
        let now = Local::now();
        let expected_filename = format!("{}-{:02}-{:02}.md", 
            now.year(), now.month(), now.day());
        
        assert!(file_path.to_string_lossy().ends_with(&expected_filename));
    }
}