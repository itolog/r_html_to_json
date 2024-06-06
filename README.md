# Rust based html parser(WASM).

## Usage 
### html_to_json
```javascript
import { html_to_json } from "r_html_to_json";


const html = `<!doctype html>
    <html lang="en">
        <head>
            <meta charset="utf-8">
            <title>Html parser</title>
        </head>
        <body>
            <h1 data-data="data" id="a" class="b c">Hello world</h1>
        </body>
    </html>`;

const result = await html_to_json(html);

console.log(result);
```

## Output

```json
{
  "tag": "html",
  "attributes": {},
  "children": [
    {
      "tag": "head",
      "attributes": {},
      "children": [
        {
          "tag": "",
          "attributes": {},
          "children": [],
          "text": "\n            "
        },
        {
          "tag": "meta",
          "attributes": {},
          "children": []
        },
        {
          "tag": "",
          "attributes": {},
          "children": [],
          "text": "\n            "
        },
        {
          "tag": "title",
          "attributes": {},
          "children": [
            {
              "tag": "",
              "attributes": {},
              "children": [],
              "text": "Html parser"
            }
          ]
        },
        {
          "tag": "",
          "attributes": {},
          "children": [],
          "text": "\n        "
        }
      ]
    },
    {
      "tag": "",
      "attributes": {},
      "children": [],
      "text": "\n        "
    },
    {
      "tag": "body",
      "attributes": {},
      "children": [
        {
          "tag": "",
          "attributes": {},
          "children": [],
          "text": "\n            "
        },
        {
          "tag": "h1",
          "attributes": {},
          "children": [
            {
              "tag": "",
              "attributes": {},
              "children": [],
              "text": "Hello world"
            }
          ]
        },
        {
          "tag": "",
          "attributes": {},
          "children": [],
          "text": "\n        \n    "
        }
      ]
    }
  ]
}
```

### json_to_html
```javascript
import { json_to_html, html_to_json } from "r_html_to_json";

const html = `<!doctype html>
    <html lang="en">
        <head>
            <meta charset="utf-8">
            <title>Html parser</title>
        </head>
        <body>
            <h1 data-data="data" id="a" class="b c">Hello world</h1>
        </body>
    </html>`;

const result = await html_to_json(html);

const resultHtml = await json_to_html(result);

console.log(resultHtml);
```

## Output

```html
<html lang="en"><head>
    <meta charset="utf-8">
    <title>Html parser</title>
</head>
<body>
<h1 data-data="data" class="b c" id="a">Hello world</h1>

</body></html>
```

### To use with vite, install the plugin
[vite-plugin-wasm](https://www.npmjs.com/package/vite-plugin-wasm)
 