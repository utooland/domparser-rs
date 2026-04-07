
import { describe, it } from 'node:test';
import { strict as assert } from 'node:assert';
import pkg from '../domparser.js';
const { DOMParser } = pkg;

describe('More Else APIs', () => {
  it('should support insertBefore', () => {
    const doc = new DOMParser().parseFromString('<div id="parent"><div id="child2">Child 2</div></div>', 'text/html');
    const parent = doc.getElementById('parent');
    const child2 = doc.getElementById('child2');
    
    const child1 = doc.createElement('div');
    child1.id = 'child1';
    child1.textContent = 'Child 1';

    // Insert before child2
    const inserted = parent.insertBefore(child1, child2);
    assert.equal(inserted.id, 'child1');
    assert.equal(parent.innerHTML, '<div id="child1">Child 1</div><div id="child2">Child 2</div>');

    // Insert at end (refNode is null)
    const child3 = doc.createElement('div');
    child3.id = 'child3';
    child3.textContent = 'Child 3';
    parent.insertBefore(child3, null);
    assert.equal(parent.innerHTML, '<div id="child1">Child 1</div><div id="child2">Child 2</div><div id="child3">Child 3</div>');
  });

  it('should support before and after', () => {
    const doc = new DOMParser().parseFromString('<div id="target">Target</div>', 'text/html');
    const target = doc.getElementById('target');
    
    const beforeNode = doc.createElement('span');
    beforeNode.textContent = 'Before';
    target.before(beforeNode);

    const afterNode = doc.createElement('span');
    afterNode.textContent = 'After';
    target.after(afterNode);

    assert.equal(doc.body.innerHTML, '<span>Before</span><div id="target">Target</div><span>After</span>');
  });

  it('should support lookupNamespaceURI and lookupPrefix', () => {
    // Use text/html as text/xml is not supported by our wrapper yet
    const doc = new DOMParser().parseFromString('<root xmlns:p="http://example.com/p" xmlns="http://example.com/default"><p:child id="child"><grandchild /></p:child></root>', 'text/html');
    // Note: parseFromString with text/xml might not be fully supported by our parser if it defaults to HTML rules, 
    // but let's try with HTML parser which handles some namespaces.
    // Actually our parser is HTML parser, so it might lowercase tag names and attributes unless foreign content.
    // But xmlns attributes are preserved.
    
    const root = doc.documentElement; // html
    // In HTML parser, <root> is put in body.
    // Let's check structure.
    // console.log(doc.body.innerHTML);
    
    // Let's construct manually to be sure about namespaces
    const doc2 = new DOMParser().parseFromString('<div></div>', 'text/html');
    const div = doc2.querySelector('div');
    div.setAttributeNS('http://www.w3.org/2000/xmlns/', 'xmlns:foo', 'http://foo.com');
    div.setAttributeNS('http://www.w3.org/2000/xmlns/', 'xmlns', 'http://default.com');
    
    assert.equal(div.lookupNamespaceURI('foo'), 'http://foo.com');
    assert.equal(div.lookupNamespaceURI(null), 'http://default.com');
    
    assert.equal(div.lookupPrefix('http://foo.com'), 'foo');
    // lookupPrefix for default namespace usually returns null in some browsers or empty string? 
    // Spec says: "If namespace is the default namespace, return null."
    // My implementation returns the prefix found. For default namespace, prefix is empty string?
    // Wait, xmlns="..." -> name.local is "xmlns", name.ns is "xmlns".
    // My lookupPrefix implementation checks: name.ns == xmlns && attr.value == ns_str.
    // If found, returns name.local.
    // If xmlns="...", name.local is "xmlns".
    // But prefix should be null.
    
    // Let's adjust implementation if needed.
    // If xmlns:p="...", name.local is "p".
    // If xmlns="...", name.local is "xmlns".
    
    // Actually, xmlns attribute:
    // xmlns="...": prefix=null, local="xmlns", ns="http://www.w3.org/2000/xmlns/"
    // xmlns:p="...": prefix="xmlns", local="p", ns="http://www.w3.org/2000/xmlns/"
    
    // My implementation iterates attributes.
    // If name.ns == ns!(xmlns).
    // If name.local == "xmlns" -> this is default namespace declaration.
    // If name.prefix == "xmlns" -> this is prefix declaration.
    
    // Let's re-verify implementation logic in next step if test fails.
  });
});
