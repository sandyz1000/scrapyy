use scrapyy::{ParseOptions, extract_from_html, extract_from_url};
use std::env;
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        // Command-line mode: extract from provided URL
        let url = &args[1];
        extract_and_display(url).await?;
    } else {
        // Demo mode: show multiple examples
        println!("🦀 Scrapyy Article Extraction Examples\n");
        println!("{}\n", "=".repeat(80));

        // Example 1: Extract from test HTML file
        example_extract_from_file().await?;

        println!("\n{}\n", "=".repeat(80));

        // Example 2: Extract from HTML
        example_extract_from_html().await?;

        println!("\n{}\n", "=".repeat(80));

        // Example 3: Custom options
        example_custom_options().await?;

        println!("\n{}\n", "=".repeat(80));

        // Example 4: Extract from URL (optional)
        example_extract_from_url().await?;

        println!("\n{}\n", "=".repeat(80));
        println!("\n💡 Tip: Run with a URL argument to extract from any webpage:");
        println!("   cargo run -- https://example.com/article\n");
    }

    Ok(())
}

async fn example_extract_from_file() -> Result<(), Box<dyn std::error::Error>> {
    println!("Example 1: Extract from HTML File (test-data)");
    println!("{}", "-".repeat(80));

    // Read test HTML file from parent directory
    let test_file = "../test-data/regular-article.html";

    match fs::read_to_string(test_file) {
        Ok(html) => {
            let url = "https://somewhere.com/path/to/article-title-here";

            println!("📄 Extracting from test file: {}\n", test_file);

            match extract_from_html(&html, url, ParseOptions::default()).await {
                Ok(article) => {
                    println!("Title: {}", article.title);
                    println!(
                        "👤 Author: {}",
                        if article.author.is_empty() {
                            "Unknown"
                        } else {
                            &article.author
                        }
                    );
                    if !article.description.is_empty() {
                        println!(
                            "📄 Description: {}",
                            &article.description.chars().take(100).collect::<String>()
                        );
                    }
                    println!("📊 Content: {} chars", article.content.len());
                    println!("🔗 Links: {}", article.links.len());
                    if !article.image.is_empty() {
                        println!("🖼️  Image: {}", article.image);
                    }
                    if !article.favicon.is_empty() {
                        println!("🎯 Favicon: {}", article.favicon);
                    }
                    println!(
                        "⏱️  Reading time: {} seconds ({} min)",
                        article.ttr,
                        article.ttr / 60
                    );

                    if article.content.len() > 0 {
                        println!("\n📖 Content Preview (first 200 chars):");
                        let preview: String = article.content.chars().take(200).collect();
                        println!("{}", preview);
                        if article.content.len() > 200 {
                            println!("...");
                        }
                    }
                }
                Err(e) => {
                    println!("❌ Error: {}", e);
                }
            }
        }
        Err(e) => {
            println!("⚠️  Could not read test file: {}", e);
            println!("   This is expected if running from a different directory.");
        }
    }

    Ok(())
}

async fn extract_and_display(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("📰 Extracting article from: {}\n", url);

    match extract_from_url(url, ParseOptions::default(), None).await {
        Ok(article) => {
            display_article(&article);
            Ok(())
        }
        Err(err) => {
            eprintln!("❌ Error extracting article: {}", err);
            Err(err.into())
        }
    }
}

fn display_article(article: &scrapyy::ParsedContent) {
    println!("Successfully extracted article\n");
    println!("Title: {}", article.title);
    println!(
        "👤 Author: {}",
        if article.author.is_empty() {
            "Unknown"
        } else {
            &article.author
        }
    );
    println!(
        "📅 Published: {}",
        if article.published.is_empty() {
            "Unknown"
        } else {
            &article.published
        }
    );
    println!(
        "🏢 Source: {}",
        if article.source.is_empty() {
            "Unknown"
        } else {
            &article.source
        }
    );
    println!(
        "📝 Type: {}",
        if article.meta_type.is_empty() {
            "article"
        } else {
            &article.meta_type
        }
    );

    if !article.description.is_empty() {
        println!("\n📄 Description:");
        println!("{}", article.description);
    }

    if !article.image.is_empty() {
        println!("\n🖼️  Image: {}", article.image);
    }

    if !article.favicon.is_empty() {
        println!("🎯 Favicon: {}", article.favicon);
    }

    println!("\n⏱️  Reading Time: {} minutes", article.ttr / 60);
    println!("📊 Content Length: {} characters", article.content.len());
    println!("🔗 Links Found: {}", article.links.len());

    if article.content.len() > 0 {
        println!("\n📖 Content Preview (first 500 chars):");
        println!("{}", "-".repeat(80));
        let preview: String = article.content.chars().take(500).collect();
        println!("{}", preview);
        if article.content.len() > 500 {
            println!("...");
        }
        println!("{}", "-".repeat(80));
    }

    if !article.links.is_empty() {
        println!("\n🔗 Sample Links (first 5):");
        for (i, link) in article.links.iter().take(5).enumerate() {
            println!("   {}. {}", i + 1, link);
        }
        if article.links.len() > 5 {
            println!("   ... and {} more", article.links.len() - 5);
        }
    }
}

async fn example_extract_from_url() -> Result<(), Box<dyn std::error::Error>> {
    println!("Example 4: Extract from URL (Optional - May Fail)");
    println!("{}", "-".repeat(80));

    let url = "https://en.wikipedia.org/wiki/Rust_(programming_language)";

    println!("📰 Extracting from: {}\n", url);

    match extract_from_url(url, ParseOptions::default(), None).await {
        Ok(article) => {
            println!("Title: {}", article.title);
            println!("📊 Content: {} chars", article.content.len());
            println!("🔗 Links: {}", article.links.len());
            println!("⏱️  Reading time: {} min", article.ttr / 60);
        }
        Err(e) => {
            println!("⚠️  Note: URL extraction failed ({})", e);
            println!("   This is normal for some websites with paywalls or bot protection.");
        }
    }

    Ok(())
}

async fn example_extract_from_html() -> Result<(), Box<dyn std::error::Error>> {
    println!("Example 2: Extract from HTML String");
    println!("{}", "-".repeat(80));

    let html = r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Sample Article - Rust Programming</title>
            <meta name="author" content="John Doe">
            <meta name="description" content="A comprehensive guide to Rust programming language.">
            <meta property="og:image" content="https://example.com/rust.png">
        </head>
        <body>
            <article>
                <h1>Introduction to Rust Programming</h1>
                <p class="author">By John Doe</p>
                <time datetime="2025-01-15">January 15, 2025</time>
                
                <p>Rust is a multi-paradigm programming language designed for performance and safety, 
                especially safe concurrency. Rust is syntactically similar to C++, but can guarantee 
                memory safety by using a borrow checker to validate references.</p>
                
                <p>Rust was originally designed by Graydon Hoare at Mozilla Research, with contributions 
                from Dave Herman, Brendan Eich, and others. The designers refined the language while 
                writing the Servo layout or browser engine, and the Rust compiler.</p>
                
                <h2>Key Features</h2>
                <ul>
                    <li>Memory safety without garbage collection</li>
                    <li>Concurrency without data races</li>
                    <li>Zero-cost abstractions</li>
                    <li>Pattern matching and type inference</li>
                </ul>
                
                <p>Rust has been voted the "most loved programming language" in the Stack Overflow 
                Developer Survey every year since 2016. It is used by companies like Mozilla, Dropbox, 
                and Cloudflare for systems programming.</p>
                
                <a href="/learn-more">Learn more about Rust</a>
                <img src="/images/rust-logo.png" alt="Rust Logo">
            </article>
        </body>
        </html>
    "#;

    let url = "https://example.com/article";

    println!("📄 Extracting from HTML string...\n");

    match extract_from_html(html, url, ParseOptions::default()).await {
        Ok(article) => {
            println!("Title: {}", article.title);
            println!("👤 Author: {}", article.author);
            println!(
                "📄 Description: {}",
                &article.description.chars().take(100).collect::<String>()
            );
            println!("📊 Content: {} chars", article.content.len());
            println!(
                "🔗 Links: {} (absolutified to {})",
                article.links.len(),
                url
            );
            println!("🖼️  Image: {}", article.image);
        }
        Err(e) => {
            println!("❌ Error: {}", e);
        }
    }

    Ok(())
}

async fn example_custom_options() -> Result<(), Box<dyn std::error::Error>> {
    println!("Example 3: Custom Parse Options");
    println!("{}", "-".repeat(80));

    let options = ParseOptions {
        words_per_minute: 300,      // Faster reading speed
        desc_truncate_len: 200,     // Shorter description
        desc_len_threshold: 100,    // Lower threshold
        content_len_threshold: 150, // Lower threshold
    };

    let html = r#"
        <html>
        <head><title>Custom Options Demo</title></head>
        <body>
            <article>
                <h1>Testing Custom Options</h1>
                <p>This example demonstrates how to use custom parsing options 
                to control the extraction behavior. You can adjust reading speed, 
                description length, and content thresholds to fit your needs.</p>
                <p>With custom options, you have fine-grained control over how 
                the article extraction works. This is useful when you need to 
                optimize for different use cases.</p>
            </article>
        </body>
        </html>
    "#;

    println!("⚙️  Custom options:");
    println!("   📚 Words per minute: {}", options.words_per_minute);
    println!(
        "   ✂️  Description max length: {}",
        options.desc_truncate_len
    );
    println!(
        "   📏 Description min threshold: {}",
        options.desc_len_threshold
    );
    println!(
        "   📏 Content min threshold: {}",
        options.content_len_threshold
    );
    println!();

    match extract_from_html(html, "https://example.com", options).await {
        Ok(article) => {
            println!("Extracted with custom options");
            println!("⏱️  Reading time (at 300 wpm): {} seconds", article.ttr);
            println!("📊 Content length: {} chars", article.content.len());
        }
        Err(e) => {
            println!("❌ Error: {}", e);
        }
    }

    Ok(())
}
