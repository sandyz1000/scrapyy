use html5ever::driver::ParseOpts;
use html5ever::tendril::TendrilSink;
use html5ever::{parse_document, serialize, Attribute};
use markup5ever_rcdom::{Handle, NodeData, RcDom};
use markup5ever::{local_name, ns};
use std::default::Default;
use std::io;
use url::{Url, ParseError};




fn normalize(html: &str, base_url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut dom = parse_document(RcDom::default(), Default::default())    
        .from_utf8()
        .read_from(&mut html.as_bytes())?;

    process_node(&dom.document, base_url)?;

    // Serialize the modified DOM back to HTML
    let mut result = Vec::new();
    serialize(&mut result, &dom.document, Default::default())?;
    Ok(String::from_utf8(result)?)
}



fn process_node(handle: &Handle, base_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let node = handle;

    match &node.data {
        NodeData::Element { ref name, ref attrs, .. } => {
            let tag_name = name.local.as_ref();
            
            // Handle `<a>` tags
            if tag_name == "a" {
                if let Some(href_attr) = attrs.borrow_mut().iter_mut().find(|attr| attr.name.local.as_ref() == "href") {
                    if let Ok(absolute_href) = absolutify(base_url, &href_attr.value) {
                        href_attr.value = absolute_href.into();
                    }
                }

                // Add target="_blank" to `<a>` tags
                attrs.borrow_mut().push(Attribute {
                    name: QualName::new(None, ns!(), local_name!("target")),
                    value: "_blank".into(),
                });
            }

            // Handle `<img>` tags
            if tag_name == "img" {
                // Prefer `data-src` over `src` if it exists
                let src_value = attrs.borrow().iter().find(|attr| attr.name.local.as_ref() == "data-src")
                    .or_else(|| attrs.borrow().iter().find(|attr| attr.name.local.as_ref() == "src"))
                    .map(|attr| attr.value.to_string());
                
                if let Some(src) = src_value {
                    if let Some(src_attr) = attrs.borrow_mut().iter_mut().find(|attr| attr.name.local.as_ref() == "src") {
                        if let Ok(absolute_src) = absolutify(base_url, &src) {
                            src_attr.value = absolute_src.into();
                        }
                    }
                }
            }
        }
        _ => {}
    }

    // Recursively process child nodes
    for child in node.children.borrow().iter() {
        process_node(child, base_url)?;
    }

    Ok(())
}


