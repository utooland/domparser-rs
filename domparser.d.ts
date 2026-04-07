/**
 * domparser-rs — Type definitions aligned to lib.dom.d.ts
 *
 * These interfaces mirror the standard DOM API hierarchy.
 * At runtime every node is a single `NodeRepr` class from the native binding,
 * but the public type surface exposes the proper DOM interfaces.
 */

// ---------------------------------------------------------------------------
// Utility types
// ---------------------------------------------------------------------------

type InsertPosition = 'beforebegin' | 'afterbegin' | 'beforeend' | 'afterend';

// ---------------------------------------------------------------------------
// DOMTokenList
// ---------------------------------------------------------------------------

export interface DOMTokenList {
  add(...tokens: string[]): void;
  remove(...tokens: string[]): void;
  toggle(token: string, force?: boolean): boolean;
  contains(token: string): boolean;
  readonly length: number;
  item(index: number): string | null;
  toString(): string;
  value: string;
  [Symbol.iterator](): Iterator<string>;
}

// ---------------------------------------------------------------------------
// Node
// ---------------------------------------------------------------------------

export interface Node {
  /** Returns an integer representing the type of the node. */
  readonly nodeType: number;
  /** Returns a string containing the name of the Node. */
  readonly nodeName: string;
  /** Returns or sets the value of the current node. */
  nodeValue: string | null;
  /** Returns or sets the text content of the node and its descendants. */
  textContent: string | null;

  /** Returns the parent of the specified node in the DOM tree. */
  readonly parentNode: Node | null;
  /** Returns the DOM node's parent Element, or null if the node either has no parent, or its parent isn't a DOM Element. */
  readonly parentElement: Element | null;
  /** Returns the first child of the node. */
  readonly firstChild: Node | null;
  /** Returns the last child of the node. */
  readonly lastChild: Node | null;
  /** Returns the node immediately preceding the specified one in its parent's childNodes list. */
  readonly previousSibling: Node | null;
  /** Returns the node immediately following the specified one in its parent's childNodes list. */
  readonly nextSibling: Node | null;
  /** Returns a live NodeList containing all the children of this node. */
  readonly childNodes: Node[];
  /** Returns the top-level document object for this node. */
  readonly ownerDocument: Document | null;

  /** Adds a node to the end of the list of children of a specified parent node. */
  appendChild<T extends Node>(node: T): T;
  /** Removes a child node from the DOM and returns the removed node. */
  removeChild<T extends Node>(child: T): T;
  /** Inserts a node before a reference node as a child of a specified parent node. */
  insertBefore<T extends Node>(node: T, child: Node | null): T;
  /** Replaces a child node within the given (parent) node. */
  replaceChild<T extends Node>(node: Node, child: T): T;
  /** Returns a duplicate of the node on which this method was called. */
  cloneNode(deep?: boolean): Node;
  /** Returns the context object's root. */
  getRootNode(): Node;
  /** Returns a boolean value indicating whether the given Node has any child nodes. */
  hasChildNodes(): boolean;
  /** Returns a boolean value indicating whether the two nodes are the same. */
  isSameNode(otherNode: Node | null): boolean;
  /** Returns a boolean value indicating whether the node is equal to the specified node. */
  isEqualNode(otherNode: Node | null): boolean;
  /** Returns a boolean value indicating whether a node is a descendant of a given node. */
  contains(other: Node | null): boolean;
  /** Puts the specified node and all of its subtree into a "normalized" form. */
  normalize(): void;
  /** Compares the position of the current node against another node in any other document. */
  compareDocumentPosition(other: Node): number;

  /** Returns the namespace URI associated with the given prefix. */
  lookupNamespaceURI(prefix: string | null): string | null;
  /** Returns the prefix for a given namespace URI, if present, and null if not. */
  lookupPrefix(namespace: string | null): string | null;
  /** Returns a boolean value indicating whether the specified namespace is the default namespace or not. */
  isDefaultNamespace(namespace: string | null): boolean;

  /** Returns a string representation of the object. */
  toString(): string;

  // Node type constants
  readonly ELEMENT_NODE: 1;
  readonly ATTRIBUTE_NODE: 2;
  readonly TEXT_NODE: 3;
  readonly CDATA_SECTION_NODE: 4;
  readonly ENTITY_REFERENCE_NODE: 5;
  readonly ENTITY_NODE: 6;
  readonly PROCESSING_INSTRUCTION_NODE: 7;
  readonly COMMENT_NODE: 8;
  readonly DOCUMENT_NODE: 9;
  readonly DOCUMENT_TYPE_NODE: 10;
  readonly DOCUMENT_FRAGMENT_NODE: 11;
  readonly NOTATION_NODE: 12;

  // Document position constants
  readonly DOCUMENT_POSITION_DISCONNECTED: 0x01;
  readonly DOCUMENT_POSITION_PRECEDING: 0x02;
  readonly DOCUMENT_POSITION_FOLLOWING: 0x04;
  readonly DOCUMENT_POSITION_CONTAINS: 0x08;
  readonly DOCUMENT_POSITION_CONTAINED_BY: 0x10;
  readonly DOCUMENT_POSITION_IMPLEMENTATION_SPECIFIC: 0x20;
}

// ---------------------------------------------------------------------------
// CharacterData
// ---------------------------------------------------------------------------

export interface CharacterData extends Node {
  /** Returns the character data of the node. */
  data: string;
  /** Returns the number of characters in the data. */
  readonly length: number;

  /** Returns a string containing the part of CharacterData.data of the specified length and starting at the specified offset. */
  substringData(offset: number, count: number): string;
  /** Appends the given string to the CharacterData.data string. */
  appendData(data: string): void;
  /** Inserts the specified string at the specified offset. */
  insertData(offset: number, data: string): void;
  /** Removes the specified amount of characters, starting at the specified offset. */
  deleteData(offset: number, count: number): void;
  /** Replaces the specified amount of characters, starting at the specified offset, with the specified string. */
  replaceData(offset: number, count: number, data: string): void;

  // ChildNode mixin
  /** Removes the object from the tree it belongs to. */
  remove(): void;
  /** Inserts a set of Node or string objects in the children list of this node's parent, just before this node. */
  before(...nodes: (Node | string)[]): void;
  /** Inserts a set of Node or string objects in the children list of this node's parent, just after this node. */
  after(...nodes: (Node | string)[]): void;
  /** Replaces this node with a set of Node or string objects. */
  replaceWith(...nodes: (Node | string)[]): void;

  // NonDocumentTypeChildNode mixin
  /** Returns the first following sibling that is an element, and null otherwise. */
  readonly nextElementSibling: Element | null;
  /** Returns the first preceding sibling that is an element, and null otherwise. */
  readonly previousElementSibling: Element | null;
}

// ---------------------------------------------------------------------------
// Text
// ---------------------------------------------------------------------------

export interface Text extends CharacterData {
  /** Breaks the Text node into two nodes at the specified offset, keeping both in the tree as siblings. */
  splitText(offset: number): Text;
}

// ---------------------------------------------------------------------------
// Comment
// ---------------------------------------------------------------------------

export interface Comment extends CharacterData {}

// ---------------------------------------------------------------------------
// ProcessingInstruction
// ---------------------------------------------------------------------------

export interface ProcessingInstruction extends CharacterData {
  /** Returns the target of the processing instruction. */
  readonly target: string;
}

// ---------------------------------------------------------------------------
// DocumentType
// ---------------------------------------------------------------------------

export interface DocumentType extends Node {
  /** Returns the name of the document type. */
  readonly name: string;
  /** Returns the public identifier of the document type. */
  readonly publicId: string;
  /** Returns the system identifier of the document type. */
  readonly systemId: string;

  // ChildNode mixin
  /** Removes the object from the tree it belongs to. */
  remove(): void;
  /** Inserts a set of Node or string objects in the children list of this node's parent, just before this node. */
  before(...nodes: (Node | string)[]): void;
  /** Inserts a set of Node or string objects in the children list of this node's parent, just after this node. */
  after(...nodes: (Node | string)[]): void;
  /** Replaces this node with a set of Node or string objects. */
  replaceWith(...nodes: (Node | string)[]): void;
}

// ---------------------------------------------------------------------------
// Element
// ---------------------------------------------------------------------------

export interface Element extends Node {
  /** Returns the name of the element. */
  readonly tagName: string;
  /** Returns the local part of the qualified name of an element. */
  readonly localName: string;
  /** Returns the namespace URI of the element, or null if the element is not in a namespace. */
  readonly namespaceURI: string | null;
  /** Returns the namespace prefix of the specified element, or null if no prefix is specified. */
  readonly prefix: string | null;

  /** Returns or sets the value of the id attribute of the element. */
  id: string;
  /** Returns or sets the value of the class attribute of the element. */
  className: string;
  /** Returns a live DOMTokenList collection of the class attributes of the element. */
  get classList(): DOMTokenList;
  /** Returns the dataset of the element. */
  readonly dataset: Record<string, string>;

  /** Returns or sets the HTML serialization of the element's descendants. */
  innerHTML: string;
  /** Returns or sets the HTML serialization of the element and its descendants. */
  outerHTML: string;

  // ParentNode mixin
  /** Returns all of the child elements of the node. */
  readonly children: Element[];
  /** Returns the number of child elements of the given element. */
  readonly childElementCount: number;
  /** Returns the first child that is an element, or null if there is none. */
  readonly firstElementChild: Element | null;
  /** Returns the last child that is an element, or null if there is none. */
  readonly lastElementChild: Element | null;
  /** Inserts a set of Node objects or string objects after the last child of the Element. */
  append(...nodes: (Node | string)[]): void;
  /** Inserts a set of Node objects or string objects before the first child of the Element. */
  prepend(...nodes: (Node | string)[]): void;
  /** Returns the first element that is a descendant of node that matches selectors. */
  querySelector(selectors: string): Element | null;
  /** Returns all element descendants of node that match selectors. */
  querySelectorAll(selectors: string): Element[];

  // NonDocumentTypeChildNode mixin
  /** Returns the Element immediately prior to the specified one in its parent's children list. */
  readonly previousElementSibling: Element | null;
  /** Returns the Element immediately following the specified one in its parent's children list. */
  readonly nextElementSibling: Element | null;

  // ChildNode mixin
  /** Removes the object from the tree it belongs to. */
  remove(): void;
  /** Inserts a set of Node or string objects in the children list of this Element's parent, just before this Element. */
  before(...nodes: (Node | string)[]): void;
  /** Inserts a set of Node or string objects in the children list of this Element's parent, just after this Element. */
  after(...nodes: (Node | string)[]): void;
  /** Replaces this Element with a set of Node or string objects. */
  replaceWith(...nodes: (Node | string)[]): void;

  // Attribute methods
  /** Returns the value of a specified attribute on the element. */
  getAttribute(qualifiedName: string): string | null;
  /** Sets the value of an attribute on the specified element. */
  setAttribute(qualifiedName: string, value: string): void;
  /** Removes an attribute from the specified element. */
  removeAttribute(qualifiedName: string): void;
  /** Toggles a boolean attribute on the given element. */
  toggleAttribute(qualifiedName: string, force?: boolean): boolean;
  /** Returns a boolean value indicating whether the specified element has the specified attribute or not. */
  hasAttribute(qualifiedName: string): boolean;
  /** Returns a boolean value indicating whether the current element has any attributes. */
  hasAttributes(): boolean;
  /** Returns the attribute names of the element as an Array of strings. */
  getAttributeNames(): string[];

  // Namespaced attribute methods
  /** Returns the string value of the attribute with the specified namespace and name. */
  getAttributeNS(namespace: string | null, localName: string): string | null;
  /** Sets the value of an attribute on the specified element with the specified namespace. */
  setAttributeNS(namespace: string | null, qualifiedName: string, value: string): void;
  /** Removes an attribute from the specified element with the specified namespace. */
  removeAttributeNS(namespace: string | null, localName: string): void;
  /** Returns a boolean value indicating whether the current element has the specified attribute. */
  hasAttributeNS(namespace: string | null, localName: string): boolean;

  // Query methods
  /** Returns an array-like object of all child elements which have all of the given class name(s). */
  getElementsByClassName(classNames: string): Element[];
  /** Returns an HTMLCollection of elements with the given tag name. */
  getElementsByTagName(qualifiedName: string): Element[];
  /** Returns the closest ancestor of the current element which matches the selectors. */
  closest(selectors: string): Element | null;
  /** Returns a boolean value indicating whether the element would be selected by the specified selector string. */
  matches(selectors: string): boolean;

  // Insertion methods
  /** Parses the specified text as HTML and inserts the resulting nodes at a specified position. */
  insertAdjacentHTML(position: InsertPosition, html: string): void;
  /** Inserts a given text node at a given position relative to the element. */
  insertAdjacentText(position: InsertPosition, text: string): void;
  /** Inserts a given element node at a given position relative to the element. */
  insertAdjacentElement(position: InsertPosition, element: Element): Element | null;

  /** Returns an Element object representing the element whose id property matches the specified string. */
  getElementById(id: string): Element | null;
}

// ---------------------------------------------------------------------------
// DocumentFragment
// ---------------------------------------------------------------------------

export interface DocumentFragment extends Node {
  // ParentNode mixin
  /** Returns all of the child elements of the fragment. */
  readonly children: Element[];
  /** Returns the number of child elements. */
  readonly childElementCount: number;
  /** Returns the first child that is an element, or null if there is none. */
  readonly firstElementChild: Element | null;
  /** Returns the last child that is an element, or null if there is none. */
  readonly lastElementChild: Element | null;
  /** Inserts nodes after the last child of the fragment. */
  append(...nodes: (Node | string)[]): void;
  /** Inserts nodes before the first child of the fragment. */
  prepend(...nodes: (Node | string)[]): void;
  /** Returns the first element that is a descendant of node that matches selectors. */
  querySelector(selectors: string): Element | null;
  /** Returns all element descendants of node that match selectors. */
  querySelectorAll(selectors: string): Element[];
  /** Returns an Element object representing the element whose id property matches the specified string. */
  getElementById(elementId: string): Element | null;
}

// ---------------------------------------------------------------------------
// Document
// ---------------------------------------------------------------------------

export interface Document extends Node {
  /** Returns the Document Type Declaration (DTD) associated with current document. */
  readonly doctype: DocumentType | null;
  /** Returns the Element that is the root element of the document. */
  readonly documentElement: Element | null;
  /** Returns the head element of the document. */
  readonly head: Element | null;
  /** Returns the body element of the document. */
  readonly body: Element | null;
  /** Returns or sets the title of the document. */
  title: string;

  // Factory methods
  /** Creates the HTML element specified by tagName. */
  createElement(tagName: string): Element;
  /** Creates a new Text node. */
  createTextNode(data: string): Text;
  /** Creates a new Comment node. */
  createComment(data: string): Comment;
  /** Creates a new empty DocumentFragment. */
  createDocumentFragment(): DocumentFragment;
  /** Creates a new ProcessingInstruction node. */
  createProcessingInstruction(target: string, data: string): ProcessingInstruction;

  /** Creates a copy of a Node from an external document that can be inserted into the current document. */
  importNode<T extends Node>(node: T, deep?: boolean): T;
  /** Transfers a node from another document into the method's document. */
  adoptNode<T extends Node>(node: T): T;

  // ParentNode mixin
  /** Returns all of the child elements of the document. */
  readonly children: Element[];
  /** Returns the number of child elements. */
  readonly childElementCount: number;
  /** Returns the first child that is an element, or null if there is none. */
  readonly firstElementChild: Element | null;
  /** Returns the last child that is an element, or null if there is none. */
  readonly lastElementChild: Element | null;
  /** Inserts nodes after the last child of the document. */
  append(...nodes: (Node | string)[]): void;
  /** Inserts nodes before the first child of the document. */
  prepend(...nodes: (Node | string)[]): void;
  /** Returns the first element that is a descendant of node that matches selectors. */
  querySelector(selectors: string): Element | null;
  /** Returns all element descendants of node that match selectors. */
  querySelectorAll(selectors: string): Element[];

  // NonElementParentNode mixin
  /** Returns an Element object representing the element whose id property matches the specified string. */
  getElementById(elementId: string): Element | null;

  // Query methods
  /** Returns an array-like object of all child elements which have all of the given class name(s). */
  getElementsByClassName(classNames: string): Element[];
  /** Returns an HTMLCollection of elements with the given tag name. */
  getElementsByTagName(qualifiedName: string): Element[];
}

// ---------------------------------------------------------------------------
// DOMParser
// ---------------------------------------------------------------------------

export class DOMParser {
  /** Parses a string containing HTML, returning a Document. */
  parseFromString(string: string, type: DOMParserSupportedType): Document;
}

type DOMParserSupportedType =
  | 'text/html'
  | 'text/xml'
  | 'application/xml'
  | 'application/xhtml+xml'
  | 'image/svg+xml';
