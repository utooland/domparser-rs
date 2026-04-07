# domparser-rs

A super fast Node.js addon for HTML parsing and manipulation, written in Rust. It provides a standard-compliant DOM API for Node.js, mirroring the browser's built-in `DOMParser`.

## Features

- **Standard Compliant**: Type definitions aligned to `lib.dom.d.ts` — uses `Node`, `Element`, `Document`, `Text`, `Comment`, and other standard DOM interfaces.
- High-performance DOM parsing and manipulation
- Exposes a simple JavaScript API via NAPI-RS
- Designed for both server-side and CLI HTML processing
- Written in Rust for speed and safety

## Installation

```bash
yarn add domparser-rs
# or
npm install domparser-rs
```

## Usage

```js
const { DOMParser } = require('domparser-rs');

const parser = new DOMParser();
const doc = parser.parseFromString('<div id="foo" class="bar">hello <span>world</span></div>', 'text/html');

const div = doc.querySelector('div');
console.log(div.getAttribute('id')); // "foo"
console.log(div.textContent); // "hello world"
div.setAttribute('title', 'my-title');
console.log(div.outerHTML); // <div id="foo" class="bar" title="my-title">hello <span>world</span></div>
```

## API Documentation

### `DOMParser`

```ts
class DOMParser {
  parseFromString(string: string, type: DOMParserSupportedType): Document;
}
```

Parses a string using the specified MIME type (e.g., `"text/html"`) and returns a `Document`.

---

### `Document`

Extends `Node`. Represents the entire HTML document.

#### Properties

| Property | Type | Description |
|---|---|---|
| `doctype` | `DocumentType \| null` | The DTD associated with the document |
| `documentElement` | `Element \| null` | The root element (e.g., `<html>`) |
| `head` | `Element \| null` | The `<head>` element |
| `body` | `Element \| null` | The `<body>` element |
| `title` | `string` | The document title |
| `children` | `Element[]` | Child elements |
| `childElementCount` | `number` | Number of child elements |
| `firstElementChild` | `Element \| null` | First child element |
| `lastElementChild` | `Element \| null` | Last child element |

#### Factory Methods

- `createElement(tagName: string): Element`
- `createTextNode(data: string): Text`
- `createComment(data: string): Comment`
- `createDocumentFragment(): DocumentFragment`
- `createProcessingInstruction(target: string, data: string): ProcessingInstruction`
- `importNode<T extends Node>(node: T, deep?: boolean): T`
- `adoptNode<T extends Node>(node: T): T`

#### Query Methods

- `getElementById(elementId: string): Element | null`
- `getElementsByClassName(classNames: string): Element[]`
- `getElementsByTagName(qualifiedName: string): Element[]`
- `querySelector(selectors: string): Element | null`
- `querySelectorAll(selectors: string): Element[]`
- `append(...nodes: (Node | string)[]): void`
- `prepend(...nodes: (Node | string)[]): void`

---

### `Element`

Extends `Node`. Represents an HTML element.

#### Properties

| Property | Type | Description |
|---|---|---|
| `tagName` | `string` | The tag name |
| `localName` | `string` | The local part of the qualified name |
| `namespaceURI` | `string \| null` | The namespace URI |
| `prefix` | `string \| null` | The namespace prefix |
| `id` | `string` | The `id` attribute |
| `className` | `string` | The `class` attribute |
| `classList` | `DOMTokenList` | Live token list of class names |
| `dataset` | `Record<string, string>` | Data attributes |
| `innerHTML` | `string` | Inner HTML content |
| `outerHTML` | `string` | Outer HTML content |
| `children` | `Element[]` | Child elements |
| `childElementCount` | `number` | Number of child elements |
| `firstElementChild` | `Element \| null` | First child element |
| `lastElementChild` | `Element \| null` | Last child element |
| `previousElementSibling` | `Element \| null` | Previous sibling element |
| `nextElementSibling` | `Element \| null` | Next sibling element |

#### Attribute Methods

- `getAttribute(qualifiedName: string): string | null`
- `setAttribute(qualifiedName: string, value: string): void`
- `removeAttribute(qualifiedName: string): void`
- `toggleAttribute(qualifiedName: string, force?: boolean): boolean`
- `hasAttribute(qualifiedName: string): boolean`
- `hasAttributes(): boolean`
- `getAttributeNames(): string[]`
- `getAttributeNS(namespace: string | null, localName: string): string | null`
- `setAttributeNS(namespace: string | null, qualifiedName: string, value: string): void`
- `removeAttributeNS(namespace: string | null, localName: string): void`
- `hasAttributeNS(namespace: string | null, localName: string): boolean`

#### Query & Selection Methods

- `querySelector(selectors: string): Element | null`
- `querySelectorAll(selectors: string): Element[]`
- `getElementById(id: string): Element | null`
- `getElementsByClassName(classNames: string): Element[]`
- `getElementsByTagName(qualifiedName: string): Element[]`
- `closest(selectors: string): Element | null`
- `matches(selectors: string): boolean`

#### Mutation Methods

- `append(...nodes: (Node | string)[]): void`
- `prepend(...nodes: (Node | string)[]): void`
- `before(...nodes: (Node | string)[]): void`
- `after(...nodes: (Node | string)[]): void`
- `remove(): void`
- `replaceWith(...nodes: (Node | string)[]): void`
- `insertAdjacentHTML(position: InsertPosition, html: string): void`
- `insertAdjacentText(position: InsertPosition, text: string): void`
- `insertAdjacentElement(position: InsertPosition, element: Element): Element | null`

---

### `Node`

Base interface for all DOM nodes.

#### Properties

| Property | Type | Description |
|---|---|---|
| `nodeType` | `number` | The type of the node |
| `nodeName` | `string` | The name of the node |
| `nodeValue` | `string \| null` | The value of the node |
| `textContent` | `string \| null` | The text content |
| `parentNode` | `Node \| null` | The parent node |
| `parentElement` | `Element \| null` | The parent element |
| `firstChild` | `Node \| null` | The first child |
| `lastChild` | `Node \| null` | The last child |
| `previousSibling` | `Node \| null` | The previous sibling |
| `nextSibling` | `Node \| null` | The next sibling |
| `childNodes` | `Node[]` | All child nodes |
| `ownerDocument` | `Document \| null` | The owner document |

#### Methods

- `appendChild<T extends Node>(node: T): T`
- `removeChild<T extends Node>(child: T): T`
- `insertBefore<T extends Node>(node: T, child: Node | null): T`
- `replaceChild<T extends Node>(node: Node, child: T): T`
- `cloneNode(deep?: boolean): Node`
- `contains(other: Node | null): boolean`
- `hasChildNodes(): boolean`
- `getRootNode(): Node`
- `normalize(): void`
- `isSameNode(otherNode: Node | null): boolean`
- `isEqualNode(otherNode: Node | null): boolean`
- `compareDocumentPosition(other: Node): number`
- `lookupNamespaceURI(prefix: string | null): string | null`
- `lookupPrefix(namespace: string | null): string | null`
- `isDefaultNamespace(namespace: string | null): boolean`

---

### `CharacterData`

Extends `Node`. Base interface for `Text`, `Comment`, and `ProcessingInstruction`.

#### Properties & Methods

- `data: string`
- `readonly length: number`
- `substringData(offset: number, count: number): string`
- `appendData(data: string): void`
- `insertData(offset: number, data: string): void`
- `deleteData(offset: number, count: number): void`
- `replaceData(offset: number, count: number, data: string): void`

### `Text` extends `CharacterData`

- `splitText(offset: number): Text`

### `Comment` extends `CharacterData`

### `ProcessingInstruction` extends `CharacterData`

- `readonly target: string`

### `DocumentType` extends `Node`

- `readonly name: string`
- `readonly publicId: string`
- `readonly systemId: string`

### `DocumentFragment` extends `Node`

- `getElementById(elementId: string): Element | null`
- `querySelector(selectors: string): Element | null`
- `querySelectorAll(selectors: string): Element[]`

---

## Contributing

```bash
npm install
npm run build
npm test
```

## Benchmark

```bash
npm run benchmark
```

---

For more usage examples and advanced API, see the source code and tests in the repository.