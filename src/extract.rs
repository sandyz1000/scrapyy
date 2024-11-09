use lazy_static::lazy_static;
use readability::extractor;
use scraper::{element_ref::ElementRef, Html, Selector};
use serde::Serialize;
use serde_json::Value;
use url::Url;
use std::{collections::HashMap, io::Cursor};

macro_rules! create_setter {
    ($field:ident, $type:ty) => {
        paste::item! {
            pub fn [<set_ $field>](&mut self, value: $type) {
                self.$field = value;
            }
        }
    };
}

fn get_meta_content(
    node: &ElementRef, attributes: &HashMap<&str, Vec<&str>>
) -> Option<(String, String)> {
    let content = node.value().attr("content")?;
    let property = node
        .value()
        .attr("property")
        .or_else(|| node.value().attr("itemprop"))
        .map(|s| s.to_lowercase());
    let name = node.value().attr("name").map(|s| s.to_lowercase());

    for (key, attrs) in attributes {
        if property
            .as_ref()
            .map_or(false, |p| attrs.contains(&p.as_str()))
            || name.as_ref().map_or(false, |n| attrs.contains(&n.as_str()))
        {
            return Some((key.to_string(), content.to_string()));
        }
    }
    None
}

// TODO: Fix this field type
#[derive(Debug, Default, Serialize, Clone)]
pub struct MetaEntry {
    pub url: String,
    pub shortlink: String,
    pub amphtml: String,
    pub canonical: String,
    pub title: String,
    pub description: String,
    pub image: String,
    pub author: String,
    pub source: String,
    pub published: String,
    pub favicon: String,
    pub meta_type: String,
}

impl MetaEntry {
    create_setter!(url, String);
    create_setter!(shortlink, String);
    create_setter!(amphtml, String);
    create_setter!(canonical, String);
    create_setter!(title, String);
    create_setter!(description, String);
    create_setter!(image, String);
    create_setter!(author, String);
    create_setter!(source, String);
    create_setter!(published, String);
    create_setter!(favicon, String);
    create_setter!(meta_type, String);
}

type FieldSetter = fn(&mut MetaEntry, String);
type FieldGetter = fn(&MetaEntry) -> &String;

lazy_static! {
    static ref SETTERS: HashMap<&'static str, FieldSetter> = {
        let mut m = HashMap::new();
        m.insert("url", MetaEntry::set_url as FieldSetter);
        m.insert("shortlink", MetaEntry::set_shortlink as FieldSetter);
        m.insert("amphtml", MetaEntry::set_amphtml as FieldSetter);
        m.insert("canonical", MetaEntry::set_canonical as FieldSetter);
        m.insert("title", MetaEntry::set_title as FieldSetter);
        m.insert("description", MetaEntry::set_description as FieldSetter);
        m.insert("image", MetaEntry::set_image as FieldSetter);
        m.insert("author", MetaEntry::set_author as FieldSetter);
        m.insert("source", MetaEntry::set_source as FieldSetter);
        m.insert("published", MetaEntry::set_published as FieldSetter);
        m.insert("favicon", MetaEntry::set_favicon as FieldSetter);
        m.insert("type", MetaEntry::set_meta_type as FieldSetter);
        m
    };
    
    static ref TYPE_SCHEMAS: Vec<String> = {
        vec![
            "aboutpage",
            "checkoutpage",
            "collectionpage",
            "contactpage",
            "faqpage",
            "itempage",
            "medicalwebpage",
            "profilepage",
            "qapage",
            "realestatelisting",
            "searchresultspage",
            "webpage",
            "website",
            "article",
            "advertisercontentarticle",
            "newsarticle",
            "analysisnewsarticle",
            "askpublicnewsarticle",
            "backgroundnewsarticle",
            "opinionnewsarticle",
            "reportagenewsarticle",
            "reviewnewsarticle",
            "report",
            "satiricalarticle",
            "scholarlyarticle",
            "medicalscholarlyarticle",
        ].iter().map(|c| c.to_string()).collect()
    };
}


const ATTRIBUTE_LISTS: &[(&str, &str)] = &[
    ("description", "description"),
    ("image", "image"),
    ("author", "author"),
    ("published", "datePublished"),
    ("type", "@type"),
];

/// Parses JSON-LD data from a document and populates an entry object.
/// Only populates if the original entry object is empty or undefined.
fn extract_ld_schema(document: &Html, entry: &mut MetaEntry) {
    // TODO: FIX ME
    // let mut entry = entry;
    if let Ok(selector) = Selector::parse(r#"script[type="application/ld+json"]"#) {
        document.select(&selector).for_each(|element|{
            if let Some(ldschema) = element.text().next() {
                let ld_json: Value = serde_json::from_str(ldschema).expect("Failed to parse JSON-LD");
            
            //     let ld_map: HashMap<String, Value> = match ld_json {
            //         Value::Object(map) => HashMap::from_iter(map),
            //         _ => panic!("Expected a JSON object"),
            //     };
            //  
            };
        }); 
    }
}

fn set_property(entry: &mut MetaEntry, field: &str, value: String) {
    if let Some(setter) = SETTERS.get(field) {
        setter(entry, value);
    }
}

// TODO: Verify and fix
pub fn extract_metadata(html: &str) -> MetaEntry {
    let mut entry = MetaEntry::default();

    let attributes = HashMap::from([
        (
            "source",
            vec![
                "application-name",
                "og:site_name",
                "twitter:site",
                "dc.title",
            ],
        ),
        ("url", vec!["og:url", "twitter:url", "parsely-link"]),
        (
            "title",
            vec!["title", "og:title", "twitter:title", "parsely-title"],
        ),
        (
            "description",
            vec![
                "description",
                "og:description",
                "twitter:description",
                "parsely-description",
            ],
        ),
        (
            "image",
            vec![
                "image",
                "og:image",
                "og:image:url",
                "og:image:secure_url",
                "twitter:image",
                "twitter:image:src",
                "parsely-image-url",
            ],
        ),
        (
            "author",
            vec![
                "author",
                "creator",
                "og:creator",
                "article:author",
                "twitter:creator",
                "dc.creator",
                "parsely-author",
            ],
        ),
        (
            "published",
            vec![
                "article:published_time",
                "article:modified_time",
                "og:updated_time",
                "dc.date",
                "dc.date.issued",
                "dc.date.created",
                "dc:created",
                "dcterms.date",
                "datepublished",
                "datemodified",
                "updated_time",
                "modified_time",
                "published_time",
                "release_date",
                "date",
                "parsely-pub-date",
            ],
        ),
        ("type", vec!["og:type"]),
    ]);

    let document = Html::parse_document(html);

    if let Some(title) = document
        .select(&Selector::parse("head > title").unwrap())
        .next()
    {
        set_property(
            &mut entry,
            "title",
            title.text().collect::<Vec<_>>().concat(),
        );
    }

    for node in document.select(&Selector::parse("link").unwrap()) {
        if let Some(rel) = node.value().attr("rel") {
            if let Some(href) = node.value().attr("href") {
                set_property(&mut entry, rel, href.to_string());
                if rel == "icon" || rel == "shortcut icon" {
                    set_property(&mut entry, "favicon", href.to_string());
                }
            }
        }
    }

    for node in document.select(&Selector::parse("meta").unwrap()) {
        if let Some((key, content)) = get_meta_content(&node, &attributes) {
            set_property(&mut entry, &key, content);
        }
    }

    extract_ld_schema(&document, &mut entry);
    entry
}


// Function to extract content with readability
pub fn extract_with_readability(html: &str, url: &str) -> Option<String> {
    
    let mut document = Cursor::new(html.as_bytes());
    let url = Url::parse(url).unwrap();

    match extractor::extract(&mut document, &url) {
        Ok(result) => {
            if !result.content.is_empty() {
                Some(result.content)
            } else {
                None
            }
        }
        Err(_) => None,
    }
}


// Function to extract title with readability
pub fn extract_title_with_readability(html: &str, url: &str) -> Option<String> {
    
    let mut document = Cursor::new(html.as_bytes());
    let url = Url::parse(url).unwrap();

    match extractor::extract(&mut document, &url) {
        Ok(result) => {
            if !result.title.is_empty() {
                Some(result.title)
            } else {
                None
            }
        }
        Err(_) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;
    use std::fs::File;
    use std::io::{BufReader, Read};


    fn read_file_by_chunks(path: &str, chunk_size: usize) -> std::io::Result<String> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        let mut buffer = vec![0; chunk_size];
        let mut contents = String::new();

        loop {
            let bytes_read = reader.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }
            contents.push_str(&String::from_utf8_lossy(&buffer[..bytes_read]));
        }

        Ok(contents)
    }

    fn read_file(path: &str) -> String {
        let mut file = File::open(path).expect("Unable to open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Unable to read file");
        contents
    }

    fn is_object(value: &Value) -> bool {
        value.is_object()
    }

    fn has_property(object: &Value, key: &str) -> bool {
        object.get(key).is_some()
    }

    #[test]
    fn test_extract_with_readability() {
        let html = r#"
            <!DOCTYPE html>
            <html>
                <head>
                <meta charset=\"UTF-8\">
                    <title>Test Page</title>
                </head>
                <body><p>This is a test page.</p></body>
            </html>
        "#;
        let url = "http://example.com";

        match extract_with_readability(html, url) {
            Some(content) => println!("Extracted content: {}", content),
            None => println!("Failed to extract content"),
        }

        match extract_title_with_readability(html, url) {
            Some(title) => println!("Extracted title: {}", title),
            None => println!("Failed to extract title"),
        }
    }

    #[test]
    fn test_extract_readability_title() {
        let html = read_file("./test-data/regular-article.html");
        let result = extract_with_readability(&html, "https://foo.bar");
        
        let Some(data) = result else {
            panic!("Error occurred while extracting HTML!!");
        };

        assert!(!data.is_empty());
        assert!(data.len() > 200);
        assert!(data.contains("<img src=\"https://foo.bar/orange.png\">"));
    }

    #[test]
    fn test_extract_readability_html() {
        let html = read_file("./test-data/regular-article.html");
        let url = "";
        let result = extract_title_with_readability(&html, url);
        assert_eq!(result.unwrap(), "Article title here - ArticleParser");
    }

    #[test]
    fn test_extract_title_badcontent() {
        let html = "<div></span>";
        let result = extract_with_readability(&html, "");
        assert_eq!(result, None);
    }

    #[test]
    fn test_extract_meta_data() {
        let html = r#"
            <!DOCTYPE html>
            <html>
                <head>
                    <meta charset=\"UTF-8\">
                    <title>Test Page</title>
                    <link rel=\"icon\" href=\"favicon.ico\">
                </head>
                <body></body>
            </html>
        "#;
        let meta_data = extract_metadata(html);
        println!("{:?}", meta_data);
    }


    #[test]
    fn test_extract_meta_data_good_content() {
        let html = read_file("./test-data/regular-article.html");
        let result = extract_metadata(&html);

        assert!(is_object(&serde_json::to_value(&result).unwrap()));
        let keys = vec![
            "url",
            "shortlink",
            "amphtml",
            "canonical",
            "title",
            "description",
            "image",
            "author",
            "source",
            "published",
            "favicon",
            "type",
        ];
        for key in keys {
            assert!(has_property(&serde_json::to_value(&result).unwrap(), key));
        }
    }

    #[test]
    fn test_extract_meta_data_json_ld_schema_content() {
        let html = read_file("./test-data/regular-article-json-ld.html");
        let result = extract_metadata(&html);

        assert!(is_object(&serde_json::to_value(&result).unwrap()));
        let keys = vec![
            "url",
            "shortlink",
            "amphtml",
            "canonical",
            "title",
            "description",
            "image",
            "author",
            "source",
            "published",
            "favicon",
            "type",
        ];
        for key in keys {
            assert!(has_property(&serde_json::to_value(&result).unwrap(), key));
        }
    }


    #[test]
    fn test_extract_ld_schema() {
        let html = r#"
        <!DOCTYPE html>
        <html>
        <head>
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Person",
                "name": "John Doe",
                "jobTitle": "Software Engineer",
                "telephone": "(425) 123-4567",
                "url": "http://www.johndoe.com"
            }
            </script>
        </head>
        <body>
            <h1>Profile</h1>
            <p>John Doe is a software engineer.</p>
        </body>
        </html>
        "#;
        let document = Html::parse_document(html);
        let meta = MetaEntry::default();


    }
    
    #[test]
    fn test_extract_from_good_html_content() {
        let html = read_file("./test-data/regular-article.html");
        let result = extract_with_readability(&html, "https://foo.bar");

        assert!(result.is_some());
        let result_str = result.unwrap();
        assert!(result_str.len() > 200);
        assert!(result_str.contains("<img src=\"https://foo.bar/orange.png\">"));
    }

    #[test]
    fn test_extract_from_bad_html_content() {
        assert!(extract_with_readability("", "").is_none());
        assert!(extract_with_readability("{}", "").is_none());
        assert!(extract_with_readability("<div></span>", "").is_none());
    }

    #[test]
    fn test_extract_title_only() {
        let html = read_file("./test-data/regular-article.html");
        let url: &str = "";
        let result = extract_title_with_readability(&html, url);
        assert_eq!(result.unwrap(), "Article title here - ArticleParser");
    }

    #[test]
    fn test_extract_title_from_page_without_title() {
        let html = read_file("./test-data/html-no-title.html");
        let url: &str = "";
        let result = extract_title_with_readability(&html, url);
        assert!(result.is_none());
    }

}
