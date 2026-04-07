use super::NodeRepr;

#[napi]
impl NodeRepr {
  /// Returns the value of a specified attribute on the element.
  #[napi]
  pub fn get_attribute(&self, name: String) -> Option<String> {
    self.0.get_attribute(name)
  }

  /// Returns the attribute names of the element as an Array of strings.
  #[napi(js_name = "getAttributeNames")]
  pub fn get_attribute_names(&self) -> Vec<String> {
    self.0.get_attribute_names()
  }

  /// Returns a boolean value indicating whether the current element has any attributes.
  #[napi(js_name = "hasAttributes")]
  pub fn has_attributes(&self) -> bool {
    self.0.has_attributes()
  }

  /// Returns a boolean value indicating whether the current Node has any child nodes.
  #[napi(js_name = "hasChildNodes")]
  pub fn has_child_nodes(&self) -> bool {
    self.0.has_child_nodes()
  }

  /// Returns the first Element within the document that matches the specified selector, or group of selectors.
  #[napi(js_name = "querySelector")]
  pub fn query_selector(&self, selectors: String) -> Option<NodeRepr> {
    self.0.query_selector(selectors).map(NodeRepr)
  }

  /// Returns a static (not live) NodeList representing a list of the document's elements that match the specified group of selectors.
  #[napi(js_name = "querySelectorAll")]
  pub fn query_selector_all(&self, selectors: String) -> Vec<NodeRepr> {
    self
      .0
      .query_selector_all(selectors)
      .into_iter()
      .map(NodeRepr)
      .collect()
  }

  /// Returns a boolean value indicating whether the specified element has the specified attribute or not.
  #[napi(js_name = "hasAttribute")]
  pub fn has_attribute(&self, name: String) -> bool {
    self.0.has_attribute(name)
  }

  /// Returns the string value of the attribute with the specified namespace and name.
  #[napi(js_name = "getAttributeNS")]
  pub fn get_attribute_ns(&self, namespace: Option<String>, local_name: String) -> Option<String> {
    self.0.get_attribute_ns(namespace, local_name)
  }

  /// Returns a boolean value indicating whether the current element has the specified attribute.
  #[napi(js_name = "hasAttributeNS")]
  pub fn has_attribute_ns(&self, namespace: Option<String>, local_name: String) -> bool {
    self.0.has_attribute_ns(namespace, local_name)
  }

  /// Returns a boolean value indicating whether the specified namespace is the default namespace or not.
  #[napi(js_name = "isDefaultNamespace")]
  pub fn is_default_namespace(&self, namespace: Option<String>) -> bool {
    self.0.is_default_namespace(namespace)
  }

  /// Returns an Element object representing the element whose id property matches the specified string.
  #[napi(js_name = "getElementById")]
  pub fn get_element_by_id(&self, id: String) -> Option<NodeRepr> {
    self.0.get_element_by_id(id).map(NodeRepr)
  }

  /// Returns an array-like object of all child elements which have all of the given class name(s).
  #[napi(js_name = "getElementsByClassName")]
  pub fn get_elements_by_class_name(&self, class_names: String) -> Vec<NodeRepr> {
    self
      .0
      .get_elements_by_class_name(class_names)
      .into_iter()
      .map(NodeRepr)
      .collect()
  }

  /// Returns an HTMLCollection of elements with the given tag name.
  #[napi(js_name = "getElementsByTagName")]
  pub fn get_elements_by_tag_name(&self, tag_name: String) -> Vec<NodeRepr> {
    self
      .0
      .get_elements_by_tag_name(tag_name)
      .into_iter()
      .map(NodeRepr)
      .collect()
  }

  /// Returns a boolean value indicating whether a node is a descendant of a given node, that is the node itself, one of its direct children (childNodes), one of the children's direct children, and so on.
  #[napi]
  pub fn contains(&self, other_node: &NodeRepr) -> bool {
    self.0.contains(&other_node.0)
  }

  /// Returns a boolean value indicating whether the node is equal to the specified node.
  #[napi(js_name = "isEqualNode")]
  pub fn is_equal_node(&self, other_node: &NodeRepr) -> bool {
    self.0.is_equal_node(&other_node.0)
  }

  /// Returns the head element of the document.
  #[napi(getter)]
  pub fn head(&self) -> Option<NodeRepr> {
    self.0.head().map(NodeRepr)
  }

  /// Returns the body element of the document.
  #[napi(getter)]
  pub fn body(&self) -> Option<NodeRepr> {
    self.0.body().map(NodeRepr)
  }

  /// Returns the title of the document.
  #[napi(getter)]
  pub fn title(&self) -> String {
    self.0.title()
  }

  /// Returns the Element that is the root element of the document (for example, the <html> element for HTML documents).
  #[napi(getter)]
  pub fn document_element(&self) -> Option<NodeRepr> {
    self.0.document_element().map(NodeRepr)
  }

  /// Returns a boolean value indicating whether the element would be selected by the specified selector string.
  #[napi(js_name = "matches")]
  pub fn matches(&self, selectors: String) -> bool {
    self.0.matches(selectors)
  }

  /// Returns the closest ancestor of the current element (or the current element itself) which matches the selectors given in parameter.
  #[napi]
  pub fn closest(&self, selectors: String) -> Option<NodeRepr> {
    self.0.closest(selectors).map(NodeRepr)
  }
}
