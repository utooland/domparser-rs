# domparser-rs

一个使用 Rust 编写的超快 Node.js HTML 解析和操作插件。它提供了一套符合标准的 DOM API，与浏览器内置的 `DOMParser` 保持一致。

## 特性

- **符合标准**：类型定义对齐 `lib.dom.d.ts` —— 使用 `Node`、`Element`、`Document`、`Text`、`Comment` 等标准 DOM 接口。
- 高性能的 DOM 解析和操作
- 通过 NAPI-RS 暴露简单的 JavaScript API
- 专为服务端和 CLI HTML 处理而设计
- 使用 Rust 编写，兼顾速度与安全性

## 安装

```bash
yarn add domparser-rs
# 或者
npm install domparser-rs
```

## 使用

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

## API 文档

### `DOMParser`

```ts
class DOMParser {
  parseFromString(string: string, type: DOMParserSupportedType): Document;
}
```

使用指定的 MIME 类型（例如 `"text/html"`）解析字符串并返回 `Document`。

---

### `Document`

继承自 `Node`。表示整个 HTML 文档。

#### 属性

| 属性 | 类型 | 描述 |
|---|---|---|
| `doctype` | `DocumentType \| null` | 当前文档关联的 DTD |
| `documentElement` | `Element \| null` | 文档的根元素（例如 `<html>`） |
| `head` | `Element \| null` | `<head>` 元素 |
| `body` | `Element \| null` | `<body>` 元素 |
| `title` | `string` | 文档标题 |
| `children` | `Element[]` | 子元素 |
| `childElementCount` | `number` | 子元素数量 |
| `firstElementChild` | `Element \| null` | 第一个子元素 |
| `lastElementChild` | `Element \| null` | 最后一个子元素 |

#### 工厂方法

- `createElement(tagName: string): Element`
- `createTextNode(data: string): Text`
- `createComment(data: string): Comment`
- `createDocumentFragment(): DocumentFragment`
- `createProcessingInstruction(target: string, data: string): ProcessingInstruction`
- `importNode<T extends Node>(node: T, deep?: boolean): T`
- `adoptNode<T extends Node>(node: T): T`

#### 查询方法

- `getElementById(elementId: string): Element | null`
- `getElementsByClassName(classNames: string): Element[]`
- `getElementsByTagName(qualifiedName: string): Element[]`
- `querySelector(selectors: string): Element | null`
- `querySelectorAll(selectors: string): Element[]`
- `append(...nodes: (Node | string)[]): void`
- `prepend(...nodes: (Node | string)[]): void`

---

### `Element`

继承自 `Node`。表示一个 HTML 元素。

#### 属性

| 属性 | 类型 | 描述 |
|---|---|---|
| `tagName` | `string` | 标签名 |
| `localName` | `string` | 限定名称的本地部分 |
| `namespaceURI` | `string \| null` | 命名空间 URI |
| `prefix` | `string \| null` | 命名空间前缀 |
| `id` | `string` | `id` 属性 |
| `className` | `string` | `class` 属性 |
| `classList` | `DOMTokenList` | 类名的实时 token 列表 |
| `dataset` | `Record<string, string>` | data 属性 |
| `innerHTML` | `string` | 内部 HTML 内容 |
| `outerHTML` | `string` | 外部 HTML 内容 |
| `children` | `Element[]` | 子元素 |
| `childElementCount` | `number` | 子元素数量 |
| `firstElementChild` | `Element \| null` | 第一个子元素 |
| `lastElementChild` | `Element \| null` | 最后一个子元素 |
| `previousElementSibling` | `Element \| null` | 前一个兄弟元素 |
| `nextElementSibling` | `Element \| null` | 后一个兄弟元素 |

#### 属性方法

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

#### 查询与选择方法

- `querySelector(selectors: string): Element | null`
- `querySelectorAll(selectors: string): Element[]`
- `getElementById(id: string): Element | null`
- `getElementsByClassName(classNames: string): Element[]`
- `getElementsByTagName(qualifiedName: string): Element[]`
- `closest(selectors: string): Element | null`
- `matches(selectors: string): boolean`

#### 操作方法

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

所有 DOM 节点的基础接口。

#### 属性

| 属性 | 类型 | 描述 |
|---|---|---|
| `nodeType` | `number` | 节点类型 |
| `nodeName` | `string` | 节点名称 |
| `nodeValue` | `string \| null` | 节点值 |
| `textContent` | `string \| null` | 文本内容 |
| `parentNode` | `Node \| null` | 父节点 |
| `parentElement` | `Element \| null` | 父元素 |
| `firstChild` | `Node \| null` | 第一个子节点 |
| `lastChild` | `Node \| null` | 最后一个子节点 |
| `previousSibling` | `Node \| null` | 前一个兄弟节点 |
| `nextSibling` | `Node \| null` | 后一个兄弟节点 |
| `childNodes` | `Node[]` | 所有子节点 |
| `ownerDocument` | `Document \| null` | 所属文档 |

#### 方法

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

继承自 `Node`。`Text`、`Comment` 和 `ProcessingInstruction` 的基础接口。

#### 属性与方法

- `data: string`
- `readonly length: number`
- `substringData(offset: number, count: number): string`
- `appendData(data: string): void`
- `insertData(offset: number, data: string): void`
- `deleteData(offset: number, count: number): void`
- `replaceData(offset: number, count: number, data: string): void`

### `Text` 继承自 `CharacterData`

- `splitText(offset: number): Text`

### `Comment` 继承自 `CharacterData`

### `ProcessingInstruction` 继承自 `CharacterData`

- `readonly target: string`

### `DocumentType` 继承自 `Node`

- `readonly name: string`
- `readonly publicId: string`
- `readonly systemId: string`

### `DocumentFragment` 继承自 `Node`

- `getElementById(elementId: string): Element | null`
- `querySelector(selectors: string): Element | null`
- `querySelectorAll(selectors: string): Element[]`

---

## 贡献

```bash
npm install
npm run build
npm test
```

## 基准测试

```bash
npm run benchmark
```

---

更多使用示例和高级 API，请参阅仓库中的源代码和测试。
