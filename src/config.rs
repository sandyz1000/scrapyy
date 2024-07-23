use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SanitizeHtmlOptions {
    pub allowed_tags: Vec<String>,
    pub allowed_attributes: HashMap<String, Vec<String>>,
    pub allowed_iframe_domains: Vec<String>,
    pub disallowed_tags_mode: String,
    pub allow_vulnerable_tags: bool,
    pub parse_style_attributes: bool,
    pub enforce_html_boundary: bool,
}

impl Default for SanitizeHtmlOptions {
    fn default() -> Self {
        Self {
            allowed_tags: vec![
                "h1", "h2", "h3", "h4", "h5", "h6",
                "u", "b", "i", "em", "strong", "small", "sup", "sub",
                "div", "span", "p", "article", "blockquote", "section",
                "details", "summary", "pre", "code", "ul", "ol", "li", "dd", "dl",
                "table", "th", "tr", "td", "thead", "tbody", "tfoot", "fieldset", "legend",
                "figure", "figcaption", "img", "picture", "video", "audio", "source",
                "iframe", "progress", "br", "p", "hr", "label", "abbr", "a", "svg",
            ].iter().map(|&s| s.to_string()).collect(),
            allowed_attributes: {
                let mut map = HashMap::new();
                map.insert("h1".to_string(), vec!["id".to_string()]);
                map.insert("h2".to_string(), vec!["id".to_string()]);
                map.insert("h3".to_string(), vec!["id".to_string()]);
                map.insert("h4".to_string(), vec!["id".to_string()]);
                map.insert("h5".to_string(), vec!["id".to_string()]);
                map.insert("h6".to_string(), vec!["id".to_string()]);
                map.insert("a".to_string(), vec!["href".to_string(), "target".to_string(), "title".to_string()]);
                map.insert("abbr".to_string(), vec!["title".to_string()]);
                map.insert("progress".to_string(), vec!["value".to_string(), "max".to_string()]);
                map.insert("img".to_string(), vec!["src".to_string(), "srcset".to_string(), "alt".to_string(), "title".to_string()]);
                map.insert("picture".to_string(), vec!["media".to_string(), "srcset".to_string()]);
                map.insert("video".to_string(), vec!["controls".to_string(), "width".to_string(), "height".to_string(), "autoplay".to_string(), "muted".to_string(), "loop".to_string(), "src".to_string()]);
                map.insert("audio".to_string(), vec!["controls".to_string(), "width".to_string(), "height".to_string(), "autoplay".to_string(), "muted".to_string(), "loop".to_string(), "src".to_string()]);
                map.insert("source".to_string(), vec!["src".to_string(), "srcset".to_string(), "data-srcset".to_string(), "type".to_string(), "media".to_string(), "sizes".to_string()]);
                map.insert("iframe".to_string(), vec!["src".to_string(), "frameborder".to_string(), "height".to_string(), "width".to_string(), "scrolling".to_string(), "allow".to_string()]);
                map.insert("svg".to_string(), vec!["width".to_string(), "height".to_string()]);
                map
            },
            allowed_iframe_domains: vec![
                "youtube.com", "vimeo.com", "odysee.com",
                "soundcloud.com", "audius.co", "github.com",
                "codepen.com", "twitter.com", "facebook.com",
                "instagram.com"
            ].iter().map(|&s| s.to_string()).collect(),
            disallowed_tags_mode: "discard".to_string(),
            allow_vulnerable_tags: false,
            parse_style_attributes: false,
            enforce_html_boundary: false,
        }
    }
}


#[derive(Clone)]
struct Config {
    sanitize_html_options: SanitizeHtmlOptions,
}

impl Config {
    fn new() -> Self {
        Self {
            sanitize_html_options: SanitizeHtmlOptions::default(),
        }
    }
}
