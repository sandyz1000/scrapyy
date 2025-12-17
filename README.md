# Scrapyy

Extract main article, main image and meta data from URL.

A Rust library for extracting clean article content from web pages, inspired by Mozilla's Readability. **Feature-complete equivalent to article-extractor!**

## ✨ Key Features

- 🚀 **Fast article extraction** using Mozilla Readability algorithm
- 📰 **Rich metadata extraction** (title, author, published date, description)
- 🖼️ **Image extraction** with main image detection
- 🔗 **URL normalization** - absolutifies all links and images **FIXED!**
- 🧹 **Clean HTML sanitization** using ammonia
- ⏱️ **Reading time estimation**
- 🌐 **Full URL support** with automatic content fetching
- 🎯 **Target="_blank"** added to external links
- 📱 **Lazy-loaded image** handling (data-src attribute)

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
scrapyy = { path = "../scrapyy" }  # or from crates.io once published
tokio = { version = "1", features = ["full"] }
```

## Usage

### Basic Example

```rust
use scrapyy::{extract_from_url, ParseOptions};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://example.com/article";
    
    let article = extract_from_url(url, ParseOptions::default(), None).await?;
    
    println!("Title: {}", article.title);
    println!("Author: {}", article.author);
    println!("Content: {}", article.content);
    
    Ok(())
}
```

### Extract from HTML String

```rust
use scrapyy::{extract_from_html, ParseOptions};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let html = r#"<html><body><article>...</article></body></html>"#;
    let url = "https://example.com";
    
    let article = extract_from_html(html, url, ParseOptions::default()).await?;
    
    println!("Title: {}", article.title);
    
    Ok(())
}
```

### Custom Parse Options

```rust
use scrapyy::ParseOptions;

let options = ParseOptions {
    words_per_minute: 250,        // Reading speed
    desc_truncate_len: 300,       // Description max length
    desc_len_threshold: 150,      // Min description length
    content_len_threshold: 200,   // Min content length
};

let article = extract_from_url(url, options, None).await?;
```

## Extracted Data Structure

```rust
pub struct ParsedContent {
    pub url: String,              // Article URL
    pub title: String,            // Article title
    pub description: String,      // Article description/summary
    pub links: Vec<String>,       // Links found in article
    pub image: String,            // Main article image
    pub content: String,          // Clean article text content
    pub author: String,           // Article author
    pub favicon: String,          // Site favicon
    pub source: String,           // Source/publisher name
    pub published: String,        // Publication date
    pub ttr: usize,              // Time to read (seconds)
    pub meta_type: String,        // Article type (article, news, etc.)
}
```

## Running Examples

```bash
cd examples
cargo run
```

## Dependencies

- `readability` - Mozilla Readability port for clean article extraction
- `scraper` - HTML parsing and CSS selector support
- `reqwest` - HTTP client for fetching URLs
- `ammonia` - HTML sanitization
- `serde` - Serialization support

## License

MIT

