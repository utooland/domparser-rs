
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

  assert.strictEqual(grandchild.isDefaultNamespace(''), true);
});

test('querySelectorAll with attributes combined', () => {
  const parser = new DOMParser();
  const doc = parser.parseFromString(`
    <div>
      <script src="app.js"></script>
      <script></script>
      <script defer src="analytics.js"></script>
      <div id="main" class="container"></div>
      <p class="text" data-id="123"></p>
      <a href="https://example.com">Link</a>
    </div>
  `, 'text/html');

  assert.strictEqual(doc.querySelectorAll('script[src]').length, 2);
  assert.strictEqual(doc.querySelectorAll('script[defer]').length, 1);
  assert.strictEqual(doc.querySelectorAll('div#main.container').length, 1);
  assert.strictEqual(doc.querySelectorAll('p.text[data-id="123"]').length, 1);
  assert.strictEqual(doc.querySelectorAll('a[href^="https"]').length, 1);
  assert.strictEqual(doc.querySelectorAll('script:not([defer])').length, 2);
});

test('querySelector with advanced pseudo-classes and combinators', () => {
  const doc = new DOMParser().parseFromString(`
    <ul id="list">
      <li>Item 1</li>
      <li class="active">Item 2</li>
      <li>Item 3</li>
      <li>Item 4</li>
    </ul>
    <div id="sibling-test">
      <h2>Heading</h2>
      <p>Paragraph 1</p>
      <p>Paragraph 2</p>
      <span>Span</span>
    </div>
  `, 'text/html');

  // :first-child / :last-child
  assert.strictEqual(doc.querySelector('#list li:first-child').textContent, 'Item 1');
  assert.strictEqual(doc.querySelector('#list li:last-child').textContent, 'Item 4');
  
  // :nth-child
  assert.strictEqual(doc.querySelector('#list li:nth-child(2)').textContent, 'Item 2');
  assert.strictEqual(doc.querySelectorAll('#list li:nth-child(odd)').length, 2);

  // adjacent sibling combinator (+)
  assert.strictEqual(doc.querySelector('.active + li').textContent, 'Item 3');
  assert.strictEqual(doc.querySelector('h2 + p').textContent, 'Paragraph 1');

  // general sibling combinator (~)
  assert.strictEqual(doc.querySelectorAll('h2 ~ p').length, 2);
  assert.strictEqual(doc.querySelectorAll('h2 ~ span').length, 1);
});
