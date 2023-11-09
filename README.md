## A simple and versatile html/xhtml parser based on Rust.

Parse html document

```rust
   import { parse_html } from "r_html_to_json";

    fn main() {
        let html = `<!doctype html>
            <html lang="en">
                <head>
                    <meta charset="utf-8">
                    <title>Html parser</title>
                </head>
                <body>
                    <h1 id="a" class="b c">Hello world</h1>
                    </h1> <!-- comments & dangling elements are ignored -->
                </body>
            </html>`;

        parse_html(html);
    }
```