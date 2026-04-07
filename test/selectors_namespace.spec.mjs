
import test from 'node:test';
import assert from 'node:assert/strict';
import pkg from '../domparser.js';
const { DOMParser } = pkg;

test('querySelector with simple selectors', () => {
  const doc = new DOMParser().parseFromString(`
    <div id="root">
      <div class="a">
        <span class="b">1</span>
        <span class="b">2</span>
      </div>
      <div class="a">
        <span class="b">3</span>
      </div>
      <div class="c">
        <span class="d">4</span>
      </div>
    </div>
  `, 'text/html');

  const root = doc.getElementById('root');

  // By class
  assert.strictEqual(root.querySelector('.b').textContent, '1');
  // By tag
  assert.strictEqual(root.querySelector('span').textContent, '1');
  // By id
  assert.strictEqual(doc.querySelector('#root').tagName, 'DIV');
  // Scoped query
  assert.strictEqual(root.querySelector('.a').querySelector('.b').textContent, '1');
  // Returns null when not found
  assert.strictEqual(root.querySelector('.nonexistent'), null);
});

test('querySelectorAll with simple selectors', () => {
  const doc = new DOMParser().parseFromString(`
    <div id="root">
      <div class="a"><span class="b">1</span><span class="b">2</span></div>
      <div class="a"><span class="b">3</span></div>
    </div>
  `, 'text/html');

  const root = doc.getElementById('root');

  assert.strictEqual(root.querySelectorAll('.b').length, 3);
  assert.strictEqual(root.querySelectorAll('.a').length, 2);
  assert.strictEqual(root.querySelectorAll('.nonexistent').length, 0);
});

test('querySelector with descendant combinator', () => {
  const doc = new DOMParser().parseFromString(`
    <div id="root">
      <div class="a">
        <span class="b">1</span>
      </div>
      <div class="c">
        <span class="d">4</span>
      </div>
    </div>
  `, 'text/html');

  const root = doc.getElementById('root');

  // Descendant combinator (space)
  const nested = root.querySelector('.c .d');
  assert.strictEqual(nested.textContent, '4');

  // Multi-level descendant
  assert.strictEqual(doc.querySelector('#root .a .b').textContent, '1');

  // querySelectorAll with descendant combinator
  assert.strictEqual(root.querySelectorAll('.a .b').length, 1);
});

test('querySelector with child combinator (>)', () => {
  const doc = new DOMParser().parseFromString(`
    <div id="root">
      <div class="parent">
        <span class="child">direct</span>
        <div class="nested">
          <span class="child">nested</span>
        </div>
      </div>
    </div>
  `, 'text/html');

  // body>* should match direct children of body
  const bodyChildren = doc.querySelectorAll('body>*');
  assert.strictEqual(bodyChildren.length, 1);
  assert.strictEqual(bodyChildren[0].id, 'root');

  // Child combinator: .parent > .child should match only direct children
  const root = doc.getElementById('root');
  const directChild = root.querySelector('.parent>.child');
  assert.strictEqual(directChild.textContent, 'direct');
});

test('querySelector with universal selector (*)', () => {
  const doc = new DOMParser().parseFromString(`
    <div id="root"><span>a</span><em>b</em></div>
  `, 'text/html');

  const root = doc.getElementById('root');

  // * matches any element
  const all = root.querySelectorAll('*');
  assert.strictEqual(all.length, 2);

  // #root > * matches direct element children
  const directChildren = doc.querySelectorAll('#root>*');
  assert.strictEqual(directChildren.length, 2);
});

test('matches with combinators', () => {
  const doc = new DOMParser().parseFromString(`
    <div id="grandparent" class="ancestor">
      <div id="parent" class="ancestor">
        <div id="child" class="target"></div>
      </div>
    </div>
  `, 'text/html');

  const child = doc.getElementById('child');

  // Simple selectors
  assert.strictEqual(child.matches('#child'), true);
  assert.strictEqual(child.matches('.target'), true);
  assert.strictEqual(child.matches('div'), true);
  assert.strictEqual(child.matches('#parent'), false);

  // Descendant combinator
  assert.strictEqual(child.matches('#parent #child'), true);
  assert.strictEqual(child.matches('.ancestor .target'), true);
  assert.strictEqual(child.matches('#grandparent #child'), true);
  assert.strictEqual(child.matches('#grandparent #parent #child'), true);

  // Child combinator
  assert.strictEqual(child.matches('#parent>#child'), true);
  assert.strictEqual(child.matches('#grandparent>#child'), false); // not direct child

  // Universal selector
  assert.strictEqual(child.matches('*'), true);
  assert.strictEqual(child.matches('#parent>*'), true);
});

test('closest with combinators', () => {
  const doc = new DOMParser().parseFromString(`
    <div id="grandparent" class="ancestor">
      <div id="parent" class="ancestor">
        <div id="child" class="target"></div>
      </div>
    </div>
  `, 'text/html');

  const child = doc.getElementById('child');

  assert.strictEqual(child.closest('.ancestor').id, 'parent');
  assert.strictEqual(child.closest('div').id, 'child');
  assert.strictEqual(child.closest('#grandparent').id, 'grandparent');
  assert.strictEqual(child.closest('.non-existent'), null);
  assert.strictEqual(child.closest('#grandparent .ancestor').id, 'parent');
});

test('non-standard APIs are not exported', () => {
  // Only DOMParser should be exported
  assert.strictEqual(typeof pkg.DOMParser, 'function');
  assert.strictEqual(pkg.parse, undefined);
  assert.strictEqual(pkg.NodeRepr, undefined);

  // Non-standard methods should not exist on nodes
  const doc = new DOMParser().parseFromString('<div>hello</div>', 'text/html');
  const div = doc.querySelector('div');
  assert.strictEqual(typeof div.select, 'undefined');
  assert.strictEqual(typeof div.selectAll, 'undefined');
  assert.strictEqual(typeof div.outerHtml, 'undefined');
  assert.strictEqual(typeof div.innerHtml, 'undefined');
  assert.strictEqual(typeof div.text, 'undefined');
  assert.strictEqual(typeof div.cloneSelfOnly, 'undefined');
  assert.strictEqual(typeof div.cloneRecursive, 'undefined');

  // Standard APIs should exist
  assert.strictEqual(typeof div.querySelector, 'function');
  assert.strictEqual(typeof div.querySelectorAll, 'function');
  assert.strictEqual(typeof div.cloneNode, 'function');
  assert.strictEqual(typeof div.outerHTML, 'string');
  assert.strictEqual(typeof div.innerHTML, 'string');
  assert.strictEqual(typeof div.textContent, 'string');
});

test('isDefaultNamespace', () => {
  const doc = new DOMParser().parseFromString(`
    <root xmlns="http://example.com/ns">
      <child xmlns="">
        <grandchild />
      </child>
    </root>
  `, 'text/html');

  const root = doc.querySelector('root');
  const child = doc.querySelector('child');
  const grandchild = doc.querySelector('grandchild');

  assert.strictEqual(root.isDefaultNamespace('http://example.com/ns'), true);
  assert.strictEqual(root.isDefaultNamespace('http://other.com'), false);

  assert.strictEqual(child.isDefaultNamespace(''), true);
  assert.strictEqual(child.isDefaultNamespace('http://example.com/ns'), false);

  // grandchild inherits from child (empty namespace)
  assert.strictEqual(grandchild.isDefaultNamespace(''), true);
});
