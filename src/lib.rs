use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{to_value, from_value};
use std::collections::HashMap;
use js_sys::Promise;

#[derive(Serialize, Deserialize, Debug)]
struct Node {
    tag: String,
    attributes: HashMap<String, String>,
    children: Vec<Node>,
    text: Option<String>,
}

#[wasm_bindgen]
pub fn html_to_json(html: String) -> Promise {
    future_to_promise(async move {
        let document = Html::parse_document(&html);
        let selector = Selector::parse("*").map_err(|e| JsValue::from_str(&e.to_string()))?;
        let root_element = document.select(&selector).next().ok_or_else(|| JsValue::from_str("No root element found"))?;

        fn element_to_node(element: scraper::ElementRef) -> Node {
            let tag = element.value().name().to_string();
            let attributes = element.value().attrs().map(|(k, v)| (k.to_string(), v.to_string())).collect();

            let children = element.children().filter_map(|child| {
                match child.value() {
                    scraper::Node::Element(_element) => {
                        let element_ref = scraper::ElementRef::wrap(child).unwrap();
                        Some(element_to_node(element_ref))
                    }
                    scraper::Node::Text(text) => Some(Node {
                        tag: String::new(),
                        attributes: HashMap::new(),
                        children: vec![],
                        text: Some(text.text.parse().unwrap()),
                    }),
                    _ => None,
                }
            }).collect();

            Node {
                tag,
                attributes,
                children,
                text: None,
            }
        }

        let root_node = element_to_node(root_element);
        let json_output = to_value(&root_node).map_err(|e| JsValue::from_str(&e.to_string()))?;
        Ok(json_output)
    })
}

#[wasm_bindgen]
pub fn json_to_html(json_value: JsValue) -> Promise {
    future_to_promise(async move {
        // Deserialize the JsValue to a Node
        let root_node: Node = from_value(json_value).map_err(|e| JsValue::from_str(&e.to_string()))?;

        // Function to convert a Node to HTML string
        fn node_to_html(node: &Node) -> String {
            let mut html = String::new();
            if !node.tag.is_empty() {
                html.push('<');
                html.push_str(&node.tag);
                for (attr, val) in &node.attributes {
                    html.push(' ');
                    html.push_str(attr);
                    html.push_str("=\"");
                    html.push_str(val);
                    html.push('"');
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
                html.push_str("</");
                html.push_str(&node.tag);
                html.push('>');
            }
            html
        }

        // Convert the root node to HTML
        let html_output = node_to_html(&root_node);
        Ok(JsValue::from_str(&html_output))
    })
}