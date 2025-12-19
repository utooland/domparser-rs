use super::NodeRepr;

#[napi]
impl NodeRepr {
  #[napi]
  pub fn select(&self, selectors: String) -> Option<NodeRepr> {
    self.0.select(selectors).map(NodeRepr)
  }

  #[napi]
  pub fn select_all(&self, selectors: String) -> Vec<NodeRepr> {
    self.0.select_all(selectors).into_iter().map(NodeRepr).collect()
  }

  #[napi]
  pub fn get_attribute(&self, name: String) -> Option<String> {
    self.0.get_attribute(name)
  }

  #[napi(js_name = "getAttributeNames")]
  pub fn get_attribute_names(&self) -> Vec<String> {
    self.0.get_attribute_names()
  }

  #[napi(js_name = "hasAttributes")]
  pub fn has_attributes(&self) -> bool {
    self.0.has_attributes()
  }

  #[napi(js_name = "hasChildNodes")]
  pub fn has_child_nodes(&self) -> bool {
    self.0.has_child_nodes()
  }

  #[napi]
  pub fn outer_html(&self) -> String {
    self.0.outer_html()
  }

  #[napi]
  pub fn inner_html(&self) -> String {
    self.0.inner_html()
  }

  #[napi]
  pub fn text(&self) -> String {
    self.0.text()
  }

  #[napi(js_name = "querySelector")]
  pub fn query_selector(&self, selectors: String) -> Option<NodeRepr> {
    self.0.query_selector(selectors).map(NodeRepr)
  }

  #[napi(js_name = "querySelectorAll")]
  pub fn query_selector_all(&self, selectors: String) -> Vec<NodeRepr> {
    self.0.query_selector_all(selectors).into_iter().map(NodeRepr).collect()
  }

  #[napi(js_name = "hasAttribute")]
  pub fn has_attribute(&self, name: String) -> bool {
    self.0.has_attribute(name)
  }

  #[napi(js_name = "getAttributeNS")]
  pub fn get_attribute_ns(&self, namespace: Option<String>, local_name: String) -> Option<String> {
    self.0.get_attribute_ns(namespace, local_name)
  }

  #[napi(js_name = "hasAttributeNS")]
  pub fn has_attribute_ns(&self, namespace: Option<String>, local_name: String) -> bool {
    self.0.has_attribute_ns(namespace, local_name)
  }

  #[napi(js_name = "isDefaultNamespace")]
  pub fn is_default_namespace(&self, namespace: Option<String>) -> bool {
    self.0.is_default_namespace(namespace)
  }

  #[napi(js_name = "getElementById")]
  pub fn get_element_by_id(&self, id: String) -> Option<NodeRepr> {
    self.0.get_element_by_id(id).map(NodeRepr)
  }

  #[napi(js_name = "getElementsByClassName")]
  pub fn get_elements_by_class_name(&self, class_names: String) -> Vec<NodeRepr> {
    self.0.get_elements_by_class_name(class_names).into_iter().map(NodeRepr).collect()
  }

  #[napi(js_name = "getElementsByTagName")]
  pub fn get_elements_by_tag_name(&self, tag_name: String) -> Vec<NodeRepr> {
    self.0.get_elements_by_tag_name(tag_name).into_iter().map(NodeRepr).collect()
  }

  #[napi]
  pub fn contains(&self, other_node: &NodeRepr) -> bool {
    self.0.contains(&other_node.0)
  }

  #[napi(js_name = "isEqualNode")]
  pub fn is_equal_node(&self, other_node: &NodeRepr) -> bool {
    self.0.is_equal_node(&other_node.0)
  }

  #[napi(getter)]
  pub fn head(&self) -> Option<NodeRepr> {
    self.0.head().map(NodeRepr)
  }

  #[napi(getter)]
  pub fn body(&self) -> Option<NodeRepr> {
    self.0.body().map(NodeRepr)
  }

  #[napi(getter)]
  pub fn title(&self) -> String {
    self.0.title()
  }

  #[napi(getter)]
  pub fn document_element(&self) -> Option<NodeRepr> {
    self.0.document_element().map(NodeRepr)
  }

  #[napi(js_name = "matches")]
  pub fn matches(&self, selectors: String) -> bool {
    self.0.matches(selectors)
  }

  #[napi]
  pub fn closest(&self, selectors: String) -> Option<NodeRepr> {
    self.0.closest(selectors).map(NodeRepr)
  }
}
