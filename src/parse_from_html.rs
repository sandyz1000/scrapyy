#![allow(unused)]

use crate::error::{AppResult as Result, Error};
use crate::{
    extract::*,
    html::{cleanify, purify},
    linker::{absolutify, is_valid_url, normalize, purify as purify_url},
    transformation::{exec_post_parser, exec_pre_parser},
    utils::get_time_to_read,
};
use reqwest::Client;

async fn fetch_html(url: &str) -> Result<String> {
    let client = Client::new();
    let res = client.get(url).send().await?;
    let body = res.text().await.map_err(|e| Error::ReqwestError(e))?;
    Ok(body)
}

fn strip_tags(html: &str) -> String {
    html2text::from_read(html.as_bytes(), 80)
}

fn summarize(description: &str, text: &str, threshold: usize, maxlen: usize) -> String {
    if description.len() > threshold {
        description.to_string()
    } else {
        let truncated = if text.len() > maxlen {
            &text[..maxlen]
        } else {
            text
        };
        truncated.replace("\n", " ")
    }
}

#[derive(Default, Debug)]
pub struct ParsedContent {
    pub url: String,
    pub title: String,
    pub description: String,
    pub links: Vec<String>,
    pub image: String,
    pub content: String,
    pub author: String,
    pub favicon: String,
    pub source: String,
    pub published: String,
    pub ttr: usize,
    pub meta_type: String,
}

#[derive(Debug)]
pub struct ParseOptions {
    pub words_per_minute: usize,
    pub desc_truncate_len: usize,
    pub desc_len_threshold: usize,
    pub content_len_threshold: usize,
}

impl ParseOptions {
    fn new(
        words_per_minute: usize,
        desc_truncate_len: usize,
        desc_len_threshold: usize,
        content_len_threshold: usize,
    ) -> Self {
        Self {
            words_per_minute,
            desc_len_threshold,
            desc_truncate_len,
            content_len_threshold,
        }
    }
}

impl Default for ParseOptions {
    fn default() -> Self {
        Self {
            words_per_minute: 300,
            desc_truncate_len: 210,
            desc_len_threshold: 180,
            content_len_threshold: 200,
        }
    }
}

pub async fn parse_from_html(
    input_html: &str,
    input_url: &str,
    parsed_options: &ParseOptions,
) -> Result<ParsedContent> {
    let pure_html = purify(input_html);
    let meta = extract_metadata(&pure_html);

    let Some(title) = extract_title_with_readability(&meta.title, "") else {
        return Err(Error::NullError(format!("Title")));
    };

    let MetaEntry {
        url,
        shortlink,
        amphtml,
        canonical,
        title,
        description,
        image,
        author,
        source,
        published,
        favicon,
        meta_type,
    } = meta.clone();
    let &ParseOptions {
        words_per_minute,
        desc_truncate_len,
        desc_len_threshold,
        content_len_threshold,
    } = parsed_options;
    let Some(title) = extract_title_with_readability(&pure_html, input_url) else {
        return Err(Error::AppError(format!(
            "Unable to extract title with readability!"
        )));
    };

    let links: Vec<String> = vec![url, shortlink, amphtml, canonical, input_url.to_string()]
        .iter()
        .filter(|u| is_valid_url(&u))
        .map(|url| purify_url(url).unwrap_or("".to_string()))
        .collect();

    if links.is_empty() {
        return Err(Error::NullError(format!("Links")));
    }

    let best_url = choose_best_url(&links, &title);
    let content = normalize(input_html, &best_url);
    let content = exec_pre_parser(&content, &links);
    let Some(content) = extract_with_readability(&content, &best_url) else {
        return Err(Error::NullError(format!("Content")));
    };
    let content = exec_post_parser(&content, &links);
    let content = cleanify(&content);

    let text_content = strip_tags(&content);

    if text_content.len() < content_len_threshold {
        return Err(Error::NullError(format!("Text content")));
    }

    let description = summarize(&meta.description, &text_content, 180, 210);
    let image = absolutify(&best_url, &image);
    let favicon = absolutify(&best_url, &favicon);
    let parsed_content = ParsedContent {
        url: best_url,
        title,
        description,
        links,
        content,
        author: meta.author,
        published: meta.published,
        image,
        favicon,
        source,
        ttr: get_time_to_read(&text_content, words_per_minute),
        meta_type,
    };

    Ok(parsed_content)
}

fn choose_best_url(links: &[String], _title: &str) -> String {
    // Implement your best URL choosing logic here
    links.first().cloned().unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};
    use std::fs;

    #[derive(Debug, Deserialize, Serialize)]
    struct Input {
        desc: String,
        html: String,
        url: Option<String>,
    }

    #[derive(Debug)]
    struct TestCase {
        input: Input,
        expectation: Option<fn(ParsedContent)>,
    }

    #[tokio::test]
    async fn test_parser() {
        let cases = vec![
            TestCase {
                input: Input {
                    desc: String::from("a webpage with no title"),
                    html: fs::read_to_string("./test-data/html-no-title.html").unwrap(),
                    url: None,
                },
                expectation: None,
            },
            TestCase {
                input: Input {
                    desc: String::from("a webpage without link"),
                    html: fs::read_to_string("./test-data/html-no-link.html").unwrap(),
                    url: None,
                },
                expectation: None,
            },
            TestCase {
                input: Input {
                    desc: String::from("a webpage with no main article"),
                    html: fs::read_to_string("./test-data/html-no-article.html").unwrap(),
                    url: None,
                },
                expectation: None,
            },
            TestCase {
                input: Input {
                    desc: String::from("a webpage with a very short article"),
                    html: fs::read_to_string("./test-data/html-too-short-article.html").unwrap(),
                    url: Some(String::from("abcd")),
                },
                expectation: None,
            },
            TestCase {
                input: Input {
                    desc: String::from("a webpage with article but no source"),
                    html: fs::read_to_string("./test-data/html-article-no-source.html").unwrap(),
                    url: None,
                },
                expectation: Some(|result| {
                    assert_eq!(result.source, "somewhere.any");
                }),
            },
            TestCase {
                input: Input {
                    desc: String::from("a webpage with data-src in img tag"),
                    html: fs::read_to_string("./test-data/html-article-with-data-src.html")
                        .unwrap(),
                    url: None,
                },
                expectation: Some(|result| {
                    let content = result.content;
                    assert!(content.contains(r#"<img src="https://somewhere.any/image1.jpg" />"#));
                    assert!(content.contains(r#"<img src="https://somewhere.any/image2.jpg" />"#));
                }),
            },
            TestCase {
                input: Input {
                    desc: String::from("a webpage with regular article"),
                    html: fs::read_to_string("./test-data/regular-article.html").unwrap(),
                    url: Some(String::from("https://somewhere.com/path/to/article")),
                },
                expectation: Some(|result| {
                    assert_eq!(result.title, "Article title here".to_owned());
                    let exp_desc = [
                    "Navigation here Few can name a rational peach that isn't a conscientious goldfish!",
                    "One cannot separate snakes from plucky pomegranates?",
                    "Draped neatly on a hanger, the melons could be said to resemble knowledgeable pigs.",
                ]
                .join(" ");

                    assert_eq!(result.description, exp_desc);
                    let content = result.content;
                    assert!(content.contains(r#"<a target="_blank" href="https://otherwhere.com/descriptions/rational-peach">"#));
                    assert!(content.contains(
                        r#"<a target="_blank" href="https://somewhere.com/dict/watermelon">"#
                    ));
                }),
            },
        ];

        for acase in cases {
            let desc = &acase.input.desc;
            let html = &acase.input.html;
            let url = acase.input.url.as_deref().unwrap_or("");
            let parsed_options = ParseOptions::default();
            let result = parse_from_html(html, url, &parsed_options).await.unwrap();
            match acase.expectation {
                Some(expectation) => {
                    expectation(result);
                }
                _ => (),
            }
        }

        let input_html = "<html>Your HTML content here</html>";
        let input_url = "https://example.com";
        let parsed_options = ParseOptions::default();
        match parse_from_html(input_html, input_url, &parsed_options).await {
            Ok(parsed_content) => {
                println!("Parsed Content: {:?}", parsed_content);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}
