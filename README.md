# A simple and versatile html/xhtml parser based on Rust.

## Usage
```
import { parse_html } from "r_html_to_json";

   
const html = `<!doctype html>
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
    
```

## Output

```
{
  "treeType": "document",
  "children": [
    {
      "name": "html",
      "variant": "normal",
      "attributes": {
        "lang": "en"
      },
      "children": [
        {
          "name": "head",
          "variant": "normal",
          "children": [
            {
              "name": "meta",
              "variant": "void",
              "attributes": {
                "charset": "utf-8"
              }
            },
            {
              "name": "title",
              "variant": "normal",
              "children": [
                "Html parser"
              ]
            }
          ]
        },
        {
          "name": "body",
          "variant": "normal",
          "children": [
            {
              "id": "a",
              "name": "h1",
              "variant": "normal",
              "classes": [
                "b",
                "c"
              ],
              "children": [
                "Hello world"
              ]
            },
            "comments & dangling elements are ignored"
          ]
        }
      ]
    }
  ]
}
```