
import test from 'node:test';
import assert from 'node:assert/strict';
import pkg from '../domparser.js';
const { DOMParser } = pkg;

test('should support cloneNode', () => {
  const doc = new DOMParser().parseFromString('<div id="parent"><span class="child">Hello</span></div>', 'text/html');
  const parent = doc.getElementById('parent');

  // Shallow clone
  const shallowClone = parent.cloneNode(false);
  assert.strictEqual(shallowClone.tagName, 'DIV');
  assert.strictEqual(shallowClone.id, 'parent');
  assert.strictEqual(shallowClone.hasChildNodes(), false);
  assert.strictEqual(shallowClone.outerHTML, '<div id="parent"></div>');

  // Deep clone
  const deepClone = parent.cloneNode(true);
  assert.strictEqual(deepClone.tagName, 'DIV');
  assert.strictEqual(deepClone.id, 'parent');
  assert.strictEqual(deepClone.hasChildNodes(), true);
  assert.strictEqual(deepClone.outerHTML, '<div id="parent"><span class="child">Hello</span></div>');
  
  // Verify independence
  deepClone.id = 'clone';
  deepClone.querySelector('span').textContent = 'World';
  assert.strictEqual(parent.id, 'parent');
  assert.strictEqual(parent.querySelector('span').textContent, 'Hello');
});

test('should support removeChild', () => {
  const doc = new DOMParser().parseFromString('<div id="parent"><span id="child1">1</span><span id="child2">2</span></div>', 'text/html');
  const parent = doc.getElementById('parent');
  const child1 = doc.getElementById('child1');
  
  const removed = parent.removeChild(child1);
  assert.strictEqual(removed.id, 'child1');
  assert.strictEqual(parent.children.length, 1);
  assert.strictEqual(parent.firstElementChild.id, 'child2');
  
  // Try to remove a node that is not a child
  const orphan = doc.createElement('div');
  assert.throws(() => {
    parent.removeChild(orphan);
  });
});

test('should support appendChild', () => {
  const doc = new DOMParser().parseFromString('<div id="parent"></div>', 'text/html');
  const parent = doc.getElementById('parent');
  
  const child1 = doc.createElement('span');
  child1.textContent = '1';
  
  const appended = parent.appendChild(child1);
  assert.strictEqual(appended, child1);
  assert.strictEqual(parent.children.length, 1);
  assert.strictEqual(parent.innerHTML, '<span>1</span>');
  
  const child2 = doc.createElement('b');
  child2.textContent = '2';
  parent.appendChild(child2);
  assert.strictEqual(parent.innerHTML, '<span>1</span><b>2</b>');
  
  // Move existing node
  parent.appendChild(child1);
  assert.strictEqual(parent.innerHTML, '<b>2</b><span>1</span>');
});

test('should support node properties', () => {
  const doc = new DOMParser().parseFromString('<div id="test">Text</div>', 'text/html');
  const div = doc.getElementById('test');
  const text = div.firstChild;
  
  assert.strictEqual(div.nodeName, 'DIV');
  assert.strictEqual(div.tagName, 'DIV');
  assert.strictEqual(div.nodeType, 1); // ELEMENT_NODE
  
  assert.strictEqual(text.nodeName, '#text');
  assert.strictEqual(text.tagName, undefined);
  assert.strictEqual(text.nodeType, 3); // TEXT_NODE
  
  assert.strictEqual(doc.nodeType, 9); // DOCUMENT_NODE
});

test('should support sibling traversal', () => {
  const doc = new DOMParser().parseFromString('<div><!-- comment --><span>text</span></div>', 'text/html');
  const div = doc.querySelector('div');
  const comment = div.firstChild;
  const span = div.lastChild;
  
  assert.strictEqual(comment.nodeType, 8); // COMMENT_NODE
  assert.strictEqual(span.nodeType, 1);
  
  assert.strictEqual(comment.nextSibling, span);
  assert.strictEqual(span.previousSibling, comment);
  
  assert.strictEqual(comment.previousSibling, null);
  assert.strictEqual(span.nextSibling, null);
});

test('should support remove', () => {
  const doc = new DOMParser().parseFromString('<div><span>remove me</span></div>', 'text/html');
  const span = doc.querySelector('span');
  
  span.remove();
  assert.strictEqual(doc.querySelector('div').innerHTML, '');
});

test('should support isEqualNode', () => {
  const doc = new DOMParser().parseFromString('<div><span>test</span></div>', 'text/html');
  const div1 = doc.querySelector('div');
  
  const doc2 = new DOMParser().parseFromString('<div><span>test</span></div>', 'text/html');
  const div2 = doc2.querySelector('div');
  
  const doc3 = new DOMParser().parseFromString('<div><span>diff</span></div>', 'text/html');
  const div3 = doc3.querySelector('div');
  
  assert.strictEqual(div1.isEqualNode(div2), true);
  assert.strictEqual(div1.isEqualNode(div3), false);
});
