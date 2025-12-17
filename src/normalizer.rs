use scraper::{Html, Selector};
use crate::linker::absolutify;
use crate::error::AppResult;

/// Normalize HTML by absolutifying URLs and adding target attributes
/// 
/// This function:
/// - Makes all <a> href attributes absolute URLs
/// - Adds target="_blank" to all links
/// - Makes all <img> src attributes absolute URLs
/// - Handles lazy-loaded images (data-src fallback)
pub fn normalize(html: &str, base_url: &str) -> AppResult<String> {
    let document = Html::parse_document(html);
    
    // Build a modified HTML string by iterating through elements
    let mut result = html.to_string();
    
    // Process all <a> tags
    let link_selector = Selector::parse("a").unwrap();
    for element in document.select(&link_selector) {
        if let Some(href) = element.value().attr("href") {
            let absolute_href = absolutify(base_url, href);
            
            // Replace href with absolute version
            let original_tag = element.html();
            let mut modified_tag = original_tag.clone();
            
            // Replace href attribute
            modified_tag = modified_tag.replace(
                &format!("href=\"{}\"", href),
                &format!("href=\"{}\"", absolute_href)
            );
            
            // Add target="_blank" if not present
            if !modified_tag.contains("target=") {
                modified_tag = modified_tag.replace(
                    "<a ",
                    "<a target=\"_blank\" "
                );
            }
            
            result = result.replace(&original_tag, &modified_tag);
        }
    }
    
    // Process all <img> tags
    let img_selector = Selector::parse("img").unwrap();
    for element in document.select(&img_selector) {
        // Check for data-src first (lazy loading), then src
        let src = element.value().attr("data-src")
            .or_else(|| element.value().attr("src"));
            
        if let Some(src_value) = src {
            let absolute_src = absolutify(base_url, src_value);
            
            let original_tag = element.html();
            let modified_tag = original_tag.replace(
                &format!("src=\"{}\"", src_value),
                &format!("src=\"{}\"", absolute_src)
            ).replace(
                &format!("data-src=\"{}\"", src_value),
                &format!("data-src=\"{}\"", absolute_src)
            );
            
            result = result.replace(&original_tag, &modified_tag);
        }
    }
    
    Ok(result)
}
