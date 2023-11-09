use html_parser::{Dom};
use wasm_bindgen::prelude::*;

extern crate web_sys;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn parse_html(html_string: &str) -> Result<String, JsError> {
    let json = Dom::parse(html_string).expect("invalid data format").to_json_pretty()?;
    Ok(json)
}
