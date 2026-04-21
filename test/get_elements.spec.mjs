import test from "node:test";
import assert from "node:assert/strict";
import pkg from "../domparser.js";
const { DOMParser } = pkg;

test("should support getElementsByTagName", () => {
  const parser = new DOMParser();
  const doc = parser.parseFromString(`
    <div>
      <p>1</p>
      <p>2</p>
      <span>3</span>
    </div>
  `, "text/html");
  const ps = doc.getElementsByTagName("p");
  assert.equal(ps.length, 2);
  assert.equal(ps[0].textContent, "1");
});

test("should support getElementsByClassName", () => {
  const parser = new DOMParser();
  const doc = parser.parseFromString(`
    <div>
      <p class="foo bar">1</p>
      <p class="foo">2</p>
      <span class="bar">3</span>
    </div>
  `, "text/html");
  const foos = doc.getElementsByClassName("foo");
  assert.equal(foos.length, 2);
  const bars = doc.getElementsByClassName("bar");
  assert.equal(bars.length, 2);
  const both = doc.getElementsByClassName("foo bar");
  assert.equal(both.length, 1);
});

test("should support getElementById", () => {
  const parser = new DOMParser();
  const doc = parser.parseFromString(`
    <div id="wrapper">
      <p id="inner">test</p>
    </div>
  `, "text/html");
  const el = doc.getElementById("inner");
  assert.ok(el);
  assert.equal(el.textContent, "test");
});
