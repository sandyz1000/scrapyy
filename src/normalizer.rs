#[allow(unused)]

use html5ever::tendril::TendrilSink;
use html5ever::driver::ParseOpts;
use html5ever::{local_name, namespace_url, ns, parse_document, serialize, Attribute, QualName};
use markup5ever_rcdom::{Handle, NodeData, RcDom, SerializableHandle};
use std::default::Default;

use crate::linker::absolutify;

pub fn normalize(html: &str, base_url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut dom = parse_document(RcDom::default(), ParseOpts::default())
        .from_utf8()
        .read_from(&mut html.as_bytes())?;

    process_node(&dom.document, base_url)?;

    // Serialize the modified DOM back to HTML
    let mut result = Vec::new();
    let doc = SerializableHandle::from(dom.document.clone());
    serialize(&mut result, &doc, Default::default())?;
    Ok(String::from_utf8(result)?)
}

fn process_node(handle: &Handle, base_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let node = handle;

    match &node.data {
        NodeData::Element {
            ref name,
            ref attrs,
            ..
        } => {
            let tag_name = name.local.as_ref();

            // Handle `<a>` tags
            if tag_name == "a" {
                if let Some(href_attr) = attrs
                    .borrow_mut()
                    .iter_mut()
                    .find(|attr| attr.name.local.as_ref() == "href")
                {
                    href_attr.value = absolutify(base_url, &href_attr.value).into();
                }

                // Add target="_blank" to `<a>` tags
                let attr = Attribute {
                    name: QualName::new(None, ns!(), local_name!("target")),
                    value: "_blank".into(),
                };
                attrs.borrow_mut().push(attr);
            }

            // Handle `<img>` tags
            if tag_name == "img" {
                // Prefer `data-src` over `src` if it exists
                let src_value = attrs
                    .borrow()
                    .clone()
                    .into_iter()
                    .find(|attr| attr.name.local.as_ref() == "data-src")
                    .or_else(|| {
                        attrs
                            .borrow()
                            .iter()
                            .find(|attr| attr.name.local.as_ref() == "src").cloned()
                    })
                    .map(|attr| attr.value.to_string());

                if let Some(src) = src_value {
                    if let Some(src_attr) = attrs
                        .borrow_mut()
                        .iter_mut()
                        .find(|attr| attr.name.local.as_ref() == "src")
                    {
                        src_attr.value = absolutify(base_url, &src).into();
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
