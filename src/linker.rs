#![allow(unused)]
use crate::similarity::find_best_match;
use scraper::{Html, Selector};

/// Check if a URL is valid
pub fn is_valid_url(url: &str) -> bool {
    url::Url::parse(url).is_ok()
}

/// Choose the best URL based on title similarity
pub fn choose_best_url(candidates: Vec<String>, title: &str) -> Option<String> {
    let best_match = find_best_match(title, &candidates);
    match best_match {
        Ok(ranking) => Some(ranking.best_match.target),
        Err(_) => None,
    }
}

/// Convert a relative URL to an absolute URL
pub fn absolutify(base_url: &str, relative_url: &str) -> String {
    match url::Url::parse(base_url) {
        Ok(base) => match base.join(relative_url) {
            Ok(url) => url.to_string(),
            Err(_) => String::new(),
        },
        Err(_) => String::new(),
    }
}

// Purify a URL by removing tracking parameters
pub fn purify(url: &str) -> Option<String> {
    if let Ok(mut parsed_url) = url::Url::parse(url) {
        let blacklist_keys = vec![
            "CNDID",
            "__twitter_impression",
            "_hsenc",
            "_openstat",
            "action_object_map",
            "action_ref_map",
            "action_type_map",
            "amp",
            "fb_action_ids",
            "fb_action_types",
            "fb_ref",
            "fb_source",
            "fbclid",
            "ga_campaign",
            "ga_content",
            "ga_medium",
            "ga_place",
            "ga_source",
            "ga_term",
            "gs_l",
            "hmb_campaign",
            "hmb_medium",
            "hmb_source",
            "mbid",
            "mc_cid",
            "mc_eid",
            "mkt_tok",
            "referrer",
            "spJobID",
            "spMailingID",
            "spReportId",
            "spUserID",
            "utm_brand",
            "utm_campaign",
            "utm_cid",
            "utm_content",
            "utm_int",
            "utm_mailing",
            "utm_medium",
            "utm_name",
            "utm_place",
            "utm_pubreferrer",
            "utm_reader",
            "utm_social",
            "utm_source",
            "utm_swu",
            "utm_term",
            "utm_userid",
            "utm_viz_id",
            "wt_mc_o",
            "yclid",
            "WT.mc_id",
            "WT.mc_ev",
            "WT.srch",
            "pk_source",
            "pk_medium",
            "pk_campaign",
        ];

        for key in blacklist_keys {
            parsed_url.query_pairs_mut().clear().append_pair(&key, "");
        }
        return Some(parsed_url.to_string());
    }
    None
}

/// Normalize URLs in HTML
pub fn normalize(html: &str, base_url: &str) -> String {
    let document = Html::parse_document(html);
    let a_selector = Selector::parse("a").unwrap();
    let img_selector = Selector::parse("img").unwrap();

    for element in document.select(&a_selector) {
        if let Some(href) = element.value().attr("href") {
            let href = absolutify(base_url, href);
            // element.set_attr("href", &absolutify(base_url, href));
            // element.set_attr("target", "_blank");
        }
    }

    for element in document.select(&img_selector) {
        if let Some(src) = element
            .value()
            .attr("data-src")
            .or(element.value().attr("src"))
        {
            let src = absolutify(base_url, src);
            // element.set_attr("src", &absolutify(base_url, src));
        }
    }

    document.root_element().html()
}

// Get the domain from a URL
pub fn get_domain(url: &str) -> Option<String> {
    url::Url::parse(url)
        .ok()
        .map(|u| u.host_str().unwrap_or("").to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    // Helper function to read file content

    fn read_file(path: &str) -> String {
        let mut file = File::open(path).expect("Unable to open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Unable to read file");
        contents
    }

    // Function to normalize URLs in HTML content
    fn normalize_urls(html: &str, base_url: &str) -> String {
        // Implement normalization logic here
        html.to_string()
    }

    #[test]
    fn test_is_valid_url() {
        let cases = vec![
            ("https://www.23hq.com", true),
            ("https://secure.actblue.com", true),
            (
                "https://docs.microsoft.com/en-us/azure/iot-edge/quickstart?view=iotedge-2018-06",
                true,
            ),
            ("http://192.168.1.199:8081/example/page", true),
            ("ftp://192.168.1.199:8081/example/page", false),
            ("", false),
            ("null", false),
            ("{\"a\": \"x\"}", false),
        ];

        for (url, expected) in cases {
            assert_eq!(is_valid_url(url), expected, "URL: {}", url);
        }
    }

    #[test]
    fn test_normalize_urls() {
        let base_url = "https://test-url.com/burritos-for-life";
        let html = read_file("./test-data/regular-article.html");
        let result = normalize_urls(&html, base_url);

        assert!(!result.contains("<a href=\"/dict/watermelon\">watermelon</a>"));
        assert!(result.contains(
            "<a target=\"_blank\" href=\"https://test-url.com/dict/watermelon\">watermelon</a>"
        ));
    }

    #[test]
    fn test_purify_url() {
        let entries = vec![
            ("", None),
            ("{}", None),
            ("https://some.where/article/abc-xyz", Some("https://some.where/article/abc-xyz")),
            ("https://some.where/article/abc-xyz#name,bob", Some("https://some.where/article/abc-xyz")),
            ("https://some.where/article/abc-xyz?utm_source=news4&utm_medium=email&utm_campaign=spring-summer", Some("https://some.where/article/abc-xyz")),
            ("https://some.where/article/abc-xyz?q=3&utm_source=news4&utm_medium=email&utm_campaign=spring-summer", Some("https://some.where/article/abc-xyz?q=3")),
            ("https://some.where/article/abc-xyz?pk_source=news4&pk_medium=email&pk_campaign=spring-summer", Some("https://some.where/article/abc-xyz")),
            ("https://some.where/article/abc-xyz?q=3&pk_source=news4&pk_medium=email&pk_campaign=spring-summer", Some("https://some.where/article/abc-xyz?q=3")),
        ];

        for (url, expected) in entries {
            assert_eq!(purify(url).unwrap(), expected.unwrap(), "URL: {}", url);
        }
    }

    #[test]
    fn test_absolutify_url() {
        let entries = vec![
            ("", "", ""),
            (
                "https://some.where/article/abc-xyz",
                "category/page.html",
                "https://some.where/article/category/page.html",
            ),
            (
                "https://some.where/article/abc-xyz",
                "../category/page.html",
                "https://some.where/category/page.html",
            ),
            (
                "https://some.where/blog/authors/article/abc-xyz",
                "/category/page.html",
                "https://some.where/category/page.html",
            ),
            (
                "https://some.where/article/abc-xyz",
                "",
                "https://some.where/article/abc-xyz",
            ),
        ];

        for (full, relative, expected) in entries {
            assert_eq!(
                absolutify(full, relative),
                expected,
                "Full URL: {}, Relative URL: {}",
                full,
                relative
            );
        }
    }

    #[test]
    fn test_choose_best_url() {
        let title = "Google đã ra giá mua Fitbit";
        let urls: Vec<String> = vec![
            "https://alpha.xyz/tin-tuc-kinh-doanh/-/view_content/content/2965950/google-da-ra-gia-mua-fitbit",
            "https://alpha.xyz/tin-tuc-kinh-doanh/view/2965950/907893219797",
            "https://alpha.xyz/tin-tuc-kinh-doanh/google-da-ra-gia-mua-fitbit",
            "https://a.xyz/read/google-da-ra-gia-mua-fitbit",
            "https://a.xyz/read/2965950/907893219797",
        ].iter().map(|u| u.to_string()).collect();

        let result = choose_best_url(urls.clone(), title);
        assert_eq!(result.unwrap(), urls[3]);
    }
}
