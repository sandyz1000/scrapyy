use lazy_static::lazy_static;
use regex::Regex;
use scraper::Html;
use std::sync::Mutex;

#[derive(Clone)]
pub struct Transformation {
    patterns: Vec<Regex>,
    pre: Option<fn(&Html) -> Html>,
    post: Option<fn(&Html) -> Html>,
}

lazy_static! {
    static ref TRANSFORMATIONS: Mutex<Vec<Transformation>> = Mutex::new(Vec::new());
}

fn add(tn: Transformation) -> usize {
    if tn.patterns.is_empty() {
        return 0;
    }
    TRANSFORMATIONS.lock().unwrap().push(tn);
    1
}

pub fn add_transformations(tfms: Vec<Transformation>) -> usize {
    tfms.into_iter()
        .map(add)
        .filter(|&result| result == 1)
        .count()
}

pub fn remove_transformations(patterns: Option<Vec<Regex>>) -> usize {
    let mut transformations = TRANSFORMATIONS.lock().unwrap();
    if patterns.is_none() {
        let removed = transformations.len();
        transformations.clear();
        return removed;
    }

    let patterns = patterns.unwrap();
    let mut removing = 0;
    transformations.retain(|transformation| {
        let matched = transformation.patterns.iter().any(|ipattern| {
            patterns
                .iter()
                .any(|pattern| pattern.as_str() == ipattern.as_str())
        });
        if matched {
            removing += 1;
            false
        } else {
            true
        }
    });
    removing
}

pub fn get_transformations() -> Vec<Transformation> {
    TRANSFORMATIONS.lock().unwrap().clone()
}

pub fn find_transformations(links: Vec<String>) -> Vec<Transformation> {
    let urls = if links.is_empty() { vec![] } else { links };
    let transformations = TRANSFORMATIONS.lock().unwrap();
    transformations
        .iter()
        .filter(|transformation| {
            urls.iter().any(|url| {
                transformation
                    .patterns
                    .iter()
                    .any(|pattern| pattern.is_match(url))
            })
        })
        .cloned()
        .collect()
}

pub fn exec_pre_parser(html: &str, links: &Vec<String>) -> String {
    let document = Html::parse_document(html);
    for transformation in find_transformations(links.clone()) {
        if let Some(pre) = transformation.pre {
            pre(&document);
        }
    }
    document.root_element().html()
}

pub fn exec_post_parser(html: &str, links: &Vec<String>) -> Option<String> {
    let document = Html::parse_document(html);
    for transformation in find_transformations(links.clone()) {
        if let Some(post) = transformation.post {
            post(&document);
        }
    }
    // document.root_element().html()
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_transformations() {
        fn pre(doc: &Html) -> Html {
            doc.clone()
        }

        fn post(doc: &Html) -> Html {
            doc.clone()
        }

        let tfms = vec![Transformation {
            patterns: vec![
                Regex::new(r"http(s?)://([\w]+\.)?def\.tld/.*").unwrap(),
            ],
            pre: Some(pre),
            post: Some(post),
        }];
        let result = add_transformations(tfms);

        assert!(result == 1)
    }

    #[test]
    fn test_find_transformations() {
        let tfms = vec![Transformation {
            patterns: vec![
                Regex::new(r"http(s?)://def\.gl/.*").unwrap(),
                Regex::new(r"http(s?)://uvw\.inc/.*").unwrap(),
            ],
            pre: None,
            post: None,
        }];
        add_transformations(tfms);

        let not_found =
            find_transformations(vec!["https://goo.gl/docs/article.html".to_string()]).is_empty();
        assert!(not_found);

        let found_one =
            find_transformations(vec!["https://lmn.inc/docs/article.html".to_string()]).len() > 1;
        assert!(found_one);

        let found_two =
            find_transformations(vec!["https://lmn.inc/docs/article.html".to_string()]).len() > 2;
        assert!(found_two);
    }

    #[test]
    fn run_exec_pre_parser() {
        let re = Regex::new(r"http(s?)://xyz\.com/.*").unwrap();
        fn pre(doc: &Html) -> Html {
            todo!()
        }

        let tfms = vec![Transformation {
            patterns: vec![re],
            pre: Some(pre),
            post: None,
        }];
        add_transformations(tfms);
        let html = r#"
        <div>
            hi <b>user</b>, this is an advertisement element
            <div class="adv">free product now!</div>
        </div>
        "#;

        let links: Vec<String> = vec!["https://xyz.com/article".to_string()];
        let result = exec_pre_parser(html, &links);

        assert!(result.contains("hi <b>user</b>, this is an advertisement element"));
        assert!(!result.contains("<div class=\"adv\">free product now!</div>"));
    }

    #[test]
    fn run_exec_post_parser() {
        let html = r#"
        <div>
            hi <b>user</b>,
            <p>Thank you for your feedback!</p>
        </div>
        "#;
        let links: Vec<String> = vec!["https://xyz.com/article".to_string()];
        if let Some(result) = exec_post_parser(html, &links) {
            assert!(result.contains("<i>user</i>"));
            assert!(!result.contains("<b>user</b>"));
        }

    }
}
