use ammonia::Builder;
use regex::Regex;
use scraper::{Html, Selector};

// Function to purify HTML
// TODO: Write test case for this function
pub fn purify(html: &str) -> String {
    let builder = Builder::new();
    builder.clean(html).to_string()
}

const WS_REGEXP: &str = r"^[\s\f\n\r\t\u1680\u180e\u2000\u2001\u2002\u2003\u2004\u2005\u2006\u2007\u2008\u2009\u200a\u2028\u2029\u202f\u205f\u3000\ufeff\x09\x0a\x0b\x0c\x0d\x20\xa0]+$";

// Function to strip multi line breaks
pub fn strip_multi_linebreaks(input: &str) -> String {
    let re = Regex::new(r"(\r\n|\n|\u2424){2,}").unwrap();
    let ws_re = Regex::new(WS_REGEXP).unwrap();
    let binding = re.replace_all(input, "\n");
    let lines: Vec<_> = binding.split('\n').collect();
    let filtered: Vec<_> = lines
        .into_iter()
        .map(|line| {
            if ws_re.is_match(line) {
                line.trim().to_string()
            } else {
                line.to_string()
            }
        })
        .filter(|line| !line.is_empty())
        .collect();
    filtered.join("\n")
}

/// Function to strip multi spaces
pub fn strip_multispaces(input: &str) -> String {
    let ws_re = Regex::new(WS_REGEXP).unwrap();
    ws_re.replace_all(input, " ").trim().to_string()
}

/// Function to get charset
pub fn get_charset(html: &str) -> String {
    let document = Html::parse_document(html);
    let meta_charset_selector = Selector::parse("meta[charset]").unwrap();
    let meta_content_type_selector = Selector::parse("meta[http-equiv=\"content-type\"]").unwrap();

    if let Some(meta) = document.select(&meta_charset_selector).next() {
        if let Some(charset) = meta.value().attr("charset") {
            return charset.to_lowercase();
        }
    }

    if let Some(meta) = document.select(&meta_content_type_selector).next() {
        if let Some(content) = meta.value().attr("content") {
            if let Some(charset) = content.split(';').nth(1) {
                return charset.replace("charset=", "").trim().to_lowercase();
            }
        }
    }

    "utf8".to_string()
}

/// Function to cleanify HTML
pub fn cleanify(input_html: &str) -> String {
    let document = Html::parse_document(input_html);
    let html_selector = Selector::parse("html").unwrap();

    let html = document.select(&html_selector).next().unwrap().inner_html();

    let sanitized = Builder::new().clean(&html).to_string();
    let cleaned = strip_multi_linebreaks(&sanitized);
    strip_multispaces(&cleaned)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Read;


    #[test]
    fn test_html() {
        let html = r#"
            <!DOCTYPE html>
            <html>
            <head>
                <meta charset=\"UTF-8\">
            </head>
            <body>    
            <p>Example   content with multiple    spaces and line breaks.</p>
            <p>Another paragraph.</p>
            </body>
            </html>
        "#;

        println!("Purified HTML: {}", purify(html));
        println!("Charset: {}", get_charset(html));
        println!("Cleanified HTML: {}", cleanify(html));
    }


    fn read_file(path: &str) -> String {
        let mut file = File::open(path).expect("Unable to open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Unable to read file");
        contents
    }

    /// ### Removes unwanted elements attributes
    #[test]
    fn test_cleanify() {
        let html = read_file("./test-data/regular-article.html");

        // Check initial HTML content
        assert!(html.contains("<address>4746 Kelly Drive, West Virginia</address>"));
        assert!(html.contains("<img src=\"./orange.png\" style=\"border: solid 1px #000\">"));

        let result = cleanify(&html);

        assert!(!result.contains("<address>4746 Kelly Drive, West Virginia</address>"));
        assert!(!result.contains("<img src=\"./orange.png\" style=\"border: solid 1px #000\">"));
    }
}
