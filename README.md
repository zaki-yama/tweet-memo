# tweet-memo

A CLI tool to record Twitter-style short memos in Markdown files.

## Installation

```bash
cargo install tweet-memo
```

## Usage

Record a memo:
```bash
tm "Your memo text here"
```

The memo will be saved to a Markdown file with timestamp in the configured format.

## Configuration

On first run, tweet-memo will create a configuration file at `~/.config/tweet-memo/config.toml` with these default settings:

- **target_directory**: Current directory
- **filename_format**: `YYYY-MM-DD.md`
- **entry_format**: `[HH:mm:ss] {text}`
- **target_section**: `### Tweets`

## Example

```bash
tm "Just had a great idea for a new feature"
```

This creates an entry like:
```markdown
### Tweets

[14:30:15] Just had a great idea for a new feature
```

## License

MIT