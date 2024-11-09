pub mod extract;
mod html;
mod linker;
mod parse_from_html;
mod retrieve;
// mod normalize;

pub mod similarity;
pub mod transformation;
mod utils;
mod error;
pub mod config;
use html::get_charset;
use linker::is_valid_url;
use encoding_rs::Encoding;
pub use parse_from_html::{parse_from_html, ParseOptions, ParsedContent};
pub use retrieve::{retrieve, RetrieveOptions};
use error::{Error, AppResult as Result};

fn buffer_to_string(buffer: &Vec<u8>) -> String {
    let text = String::from_utf8_lossy(&buffer);
    text.trim().to_string()
}

pub async fn extract(
    input_url: &str,
    parser_opts: ParseOptions,
    fetch_opts: Option<RetrieveOptions>,
) -> Result<ParsedContent> {
    if !is_valid_url(input_url) {
        let parsed = 
            parse_from_html("", input_url, &parser_opts).await;
        return parsed;
    }
    let buffer: Vec<u8> = retrieve::retrieve(input_url, fetch_opts).await?;
    let text = buffer_to_string(&buffer);
    if text.is_empty() {
        // return null
    }
    let charset = get_charset(&text);
    let (html, _, _) = if let Some(encoding) = Encoding::for_label(charset.as_bytes()) {
        encoding.decode(&buffer)
    } else {
        panic!("Unsupported encoding: {}", charset);
    };
    
    let parsed = 
        parse_from_html(&html, input_url, &parser_opts).await;
    
    parsed
}

pub async fn extract_from_html(
    html: &str,
    url: &str,
    parser_opts: ParseOptions,
) -> Result<ParsedContent> {
    let parsed = parse_from_html(html, url, &parser_opts).await;
    parsed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_extract() {
        let input = "https://www.cnbc.com/2022/09/21/what-another-major-rate-hike-by-the-federal-reserve-means-to-you.html";
        let parser_opts = ParseOptions::default();
        match extract(input, parser_opts, None).await {
            Ok(article) => println!("{:#?}", article),
            Err(err) => eprintln!("Error: {}", err),
        }
    }
}
