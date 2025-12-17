use scrapyy::{ParseOptions, extract_from_url};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    let url = if args.len() > 1 {
        &args[1]
    } else {
        // Default to a well-known article URL that works well with readability
        "https://en.wikipedia.org/wiki/Web_scraping"
    };

    println!("🦀 Scrapyy - Article Extraction Example\n");
    println!("Extracting from: {}\n", url);

    match extract_from_url(url, ParseOptions::default(), None).await {
        Ok(article) => {
            println!("SUCCESS!\n");
            println!("Title: {}", article.title);
            println!(
                "👤 Author: {}",
                if article.author.is_empty() {
                    "Unknown"
                } else {
                    &article.author
                }
            );
            let preview = article.description.chars().take(150).collect::<String>();
            println!(
                "📄 Description: {}",
                if article.description.is_empty() {
                    "None"
                } else {
                    &preview
                }
            );
            println!("📊 Content Length: {} characters", article.content.len());
            println!("🔗 Links Found: {}", article.links.len());
            println!(
                "⏱️  Reading Time: {} min {} sec",
                article.ttr / 60,
                article.ttr % 60
            );

            if !article.image.is_empty() {
                println!("🖼️  Image: {}", article.image);
            }

            if article.content.len() > 0 {
                println!("\n📖 Content Preview:");
                println!("{}", "-".repeat(80));
                println!("{}", article.content.chars().take(300).collect::<String>());
                println!("...");
                println!("{}", "-".repeat(80));
            }
        }
        Err(e) => {
            eprintln!("❌ Error: {}", e);
            eprintln!("\n💡 Tips:");
            eprintln!("   - Make sure the URL is a valid article page, not a listing/index page");
            eprintln!("   - The page should have actual article content (paragraphs of text)");
            eprintln!("   - Some sites may block web scraping");
            eprintln!("\n✨ Try with these example URLs:");
            eprintln!(
                "   cargo run --example simple -- https://en.wikipedia.org/wiki/Web_scraping"
            );
            eprintln!(
                "   cargo run --example simple -- https://en.wikipedia.org/wiki/Rust_(programming_language)"
            );
            return Err(e.into());
        }
    }

    Ok(())
}
