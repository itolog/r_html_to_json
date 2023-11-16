import './style.css'
import typescriptLogo from './typescript.svg'
import viteLogo from '/vite.svg'
import { setupCounter } from './counter.ts'
import {parse_html} from "r_html_to_json";
import { htmlStr3} from "./data/data.ts";
import { HTMLToJSON } from 'html-to-json-parser';

console.time("parse_html");
const r1 = parse_html(htmlStr3);
console.log(r1);
console.timeEnd("parse_html");
console.log('=====================');

console.time("HTMLToJSON");
let result = await HTMLToJSON(htmlStr3, true);
console.log(result);
console.timeEnd("HTMLToJSON");


document.querySelector<HTMLDivElement>('#app')!.innerHTML = `
  <div>
    <a href="https://vitejs.dev" target="_blank">
      <img src="${viteLogo}" class="logo" alt="Vite logo" />
    </a>
    <a href="https://www.typescriptlang.org/" target="_blank">
      <img src="${typescriptLogo}" class="logo vanilla" alt="TypeScript logo" />
    </a>
    <h1>Vite + TypeScript</h1>
    <div class="card">
      <button id="counter" type="button"></button>
    </div>
    <p class="read-the-docs">
      Click on the Vite and TypeScript logos to learn more
    </p>
  </div>
`

setupCounter(document.querySelector<HTMLButtonElement>('#counter')!)
