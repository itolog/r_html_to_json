import './style.css'
import typescriptLogo from './typescript.svg'
import viteLogo from '/vite.svg'
import { setupCounter } from './counter.ts'
// @ts-ignore
import {html_to_json, json_to_html} from "r_html_to_json";
import {htmlStr} from "./data/data.ts";
// @ts-ignore
import { HTMLToJSON } from 'html-to-json-parser';

const startTime = performance.now();
const r1 = await html_to_json(htmlStr);
const endTime = performance.now();
console.log(`RUST Function took ${endTime - startTime} milliseconds to execute.`);

console.log('=====================');
const startTime3 = performance.now();
try {
    console.log(r1)
    const as= await json_to_html( r1)
    console.log('TO HTML',as)
}catch (e) {
    console.log("EROROR",e)
}

const endTime3 = performance.now();
console.log(`RUST Function took ${endTime3 - startTime3} milliseconds to execute.`);

const startTime2 = performance.now();
let result = await HTMLToJSON(htmlStr, true);
const endTime2 = performance.now();
console.log(`JS Function took ${endTime2 - startTime2} milliseconds to execute.`);
console.log(JSON.parse(result))

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
