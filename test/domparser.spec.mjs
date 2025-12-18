import test from 'node:test'
import assert from 'node:assert/strict'
import { DOMParser } from '../domparser.js'

test('DOMParser should parse html string', () => {
  const parser = new DOMParser()
  const doc = parser.parseFromString('<div></div>', 'text/html')
  assert.strictEqual(
    doc.documentElement.outerHTML,
    '<html><head></head><body><div></div></body></html>',
  )
})

test('DOMParser should throw on unsupported mime type', () => {
  const parser = new DOMParser()
  assert.throws(
    () => {
      parser.parseFromString('<div></div>', 'text/xml')
    },
    { message: 'Unsupported mime type: text/xml' },
  )
})

test('should support innerHTML setter', () => {
  const parser = new DOMParser()
  const doc = parser.parseFromString('<div></div>', 'text/html')
  const div = doc.querySelector('div')
  div.innerHTML = '<span>hello</span>'
  assert.strictEqual(div.innerHTML, '<span>hello</span>')
})

test('should support textContent setter', () => {
  const parser = new DOMParser()
  const doc = parser.parseFromString('<div></div>', 'text/html')
  const div = doc.querySelector('div')
  div.textContent = 'hello'
  assert.strictEqual(div.innerHTML, 'hello')
})

test('should support replaceChild', () => {
  const parser = new DOMParser()
  const doc = parser.parseFromString('<div><span>old</span></div>', 'text/html')
  const div = doc.querySelector('div')
  const oldSpan = doc.querySelector('span')
  const newSpan = doc.createElement('b')
  newSpan.textContent = 'new'
  
  div.replaceChild(newSpan, oldSpan)
  assert.strictEqual(div.innerHTML, '<b>new</b>')
})

test('should support contains', () => {
  const parser = new DOMParser()
  const doc = parser.parseFromString('<div><span>hello</span></div>', 'text/html')
  const div = doc.querySelector('div')
  const span = doc.querySelector('span')
  assert.ok(div.contains(span))
  assert.ok(doc.contains(span))
  assert.strictEqual(span.contains(div), false)
})

test('should support head/body/title', () => {
  const parser = new DOMParser()
  const doc = parser.parseFromString('<title>Test</title><div></div>', 'text/html')
  assert.ok(doc.head)
  assert.ok(doc.body)
  assert.strictEqual(doc.title, 'Test')
})

test('should support element traversal', () => {
  const parser = new DOMParser()
  const doc = parser.parseFromString('<div><span>1</span><b>2</b></div>', 'text/html')
  const div = doc.querySelector('div')
  const span = doc.querySelector('span')
  const b = doc.querySelector('b')

  assert.strictEqual(div.firstElementChild.outerHTML, '<span>1</span>')
  assert.strictEqual(div.lastElementChild.outerHTML, '<b>2</b>')
  assert.strictEqual(span.nextElementSibling.outerHTML, '<b>2</b>')
  assert.strictEqual(b.previousElementSibling.outerHTML, '<span>1</span>')
  assert.strictEqual(
    span.parentElement.outerHTML,
    '<div><span>1</span><b>2</b></div>',
  )
  assert.strictEqual(div.childElementCount, 2)
  assert.strictEqual(div.children.length, 2)
})

test('should support matches and closest', () => {
  const parser = new DOMParser()
  const doc = parser.parseFromString('<div class="foo"><span id="bar">hello</span></div>', 'text/html')
  const span = doc.querySelector('#bar')
  
  assert.ok(span.matches('#bar'))
  assert.ok(span.matches('span'))
  assert.ok(span.matches('.foo span'))
  assert.strictEqual(span.matches('div'), false)
  
  const closestDiv = span.closest('div')
  assert.ok(closestDiv)
  assert.strictEqual(closestDiv.className, 'foo')
  
  const closestSpan = span.closest('span')
  assert.strictEqual(closestSpan.id, 'bar')
})

test('should support getRootNode', () => {
  const parser = new DOMParser()
  const doc = parser.parseFromString('<div></div>', 'text/html')
  const div = doc.querySelector('div')
  assert.strictEqual(div.getRootNode().nodeType, 9) // Document node
})

test('should support createComment and createDocumentFragment', () => {
  const parser = new DOMParser()
  const doc = parser.parseFromString('<div></div>', 'text/html')
  
  const comment = doc.createComment('my comment')
  assert.strictEqual(comment.nodeType, 8)
  assert.strictEqual(comment.nodeValue, 'my comment')
  
  const fragment = doc.createDocumentFragment()
  assert.strictEqual(fragment.nodeType, 11)
  fragment.append(comment)
  assert.strictEqual(fragment.childNodes.length, 1)
})

test('should support isSameNode', () => {
  const parser = new DOMParser()
  const doc = parser.parseFromString('<div></div>', 'text/html')
  const div1 = doc.querySelector('div')
  const div2 = doc.querySelector('div')
  
  assert.ok(div1.isSameNode(div2))
  assert.strictEqual(div1.isSameNode(doc), false)
})

test('should support attribute methods', () => {
  const parser = new DOMParser()
  const doc = parser.parseFromString('<div id="foo" class="bar"></div>', 'text/html')
  const div = doc.querySelector('div')
  
  assert.ok(div.hasAttributes())
  assert.deepStrictEqual(div.getAttributeNames().sort(), ['class', 'id'])
  
  div.toggleAttribute('hidden')
  assert.ok(div.hasAttribute('hidden'))
  
  div.toggleAttribute('hidden')
  assert.strictEqual(div.hasAttribute('hidden'), false)
  
  div.toggleAttribute('readonly', true)
  assert.ok(div.hasAttribute('readonly'))
  
  div.toggleAttribute('readonly', false)
  assert.strictEqual(div.hasAttribute('readonly'), false)
})

test('should support documentElement', () => {
  const parser = new DOMParser()
  const doc = parser.parseFromString('<html><body></body></html>', 'text/html')
  assert.ok(doc.documentElement)
  assert.strictEqual(doc.documentElement.tagName, 'HTML')
})
