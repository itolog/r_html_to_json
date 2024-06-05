mod utils;

use wasm_bindgen::prelude::*;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

extern crate web_sys;

#[derive(Serialize, Deserialize, Debug)]
struct Node {
    tag: String,
    attributes: HashMap<String, String>,
    children: Vec<Node>,
    text: Option<String>,
}


// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub async fn html_to_json(html: &str) -> Result<JsValue, JsValue>  {
    utils::set_panic_hook();
    let document = Html::parse_document(html);
    let selector = Selector::parse("*").unwrap();
    let root_element = document.select(&selector).next().unwrap();

    fn element_to_node(element: scraper::ElementRef) -> Node {
        let tag = element.value().name().to_string();
        let attributes = element
            .value()
            .attrs()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();

        let children = element
            .children()
            .filter_map(|child| {
                match child.value() {
                    scraper::Node::Element(_element) => Some(element_to_node(scraper::ElementRef::wrap(child).unwrap())),
                    scraper::Node::Text(text) => Some(Node {
                        tag: "".to_string(),
                        attributes: HashMap::new(),
                        children: vec![],
                        text: Some(text.text.parse().unwrap()),
                    }),
                    _ => None,
                }
            })
            .collect();

        Node {
            tag,
            attributes,
            children,
            text: None,
        }
    }

    let root_node = element_to_node(root_element);
    Ok(serde_wasm_bindgen::to_value(&root_node)?)
}

#[wasm_bindgen]
pub async fn json_to_html(value: JsValue) -> Result<JsValue, JsValue> {
    utils::set_panic_hook();

    fn node_to_html(node: &Node) -> String {
        let mut html = String::new();
        if !node.tag.is_empty() {
            html.push_str(&format!("<{}", node.tag));
            for (attr, val) in &node.attributes {
                html.push_str(&format!(" {}=\"{}\"", attr, val));
            }
            html.push('>');
        }

        if let Some(text) = &node.text {
            html.push_str(text);
        }

        for child in &node.children {
            html.push_str(&node_to_html(child));
        }

        if !node.tag.is_empty() {
            html.push_str(&format!("</{}>", node.tag));
        }
        html
    }

    let root_node: Node = serde_wasm_bindgen::from_value(value).expect("ERERRER");

    Ok(serde_wasm_bindgen::to_value(&node_to_html(&root_node))?)
}