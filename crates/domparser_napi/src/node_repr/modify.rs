use super::NodeRepr;

#[napi]
impl NodeRepr {
  #[napi]
  pub fn append(&self, new_child: &NodeRepr) {
    self.0.append(&new_child.0);
  }

  #[napi(js_name = "appendChild")]
  pub fn append_child(&self, new_child: &NodeRepr) -> NodeRepr {
    NodeRepr(self.0.append_child(&new_child.0))
  }

  #[napi(js_name = "removeChild")]
  pub fn remove_child(&self, child: &NodeRepr) -> NodeRepr {
    NodeRepr(self.0.remove_child(&child.0))
  }

  #[napi]
  pub fn prepend(&self, new_child: &NodeRepr) {
    self.0.prepend(&new_child.0);
  }

  #[napi(js_name = "after")]
  pub fn after(&self, new_sibling: &NodeRepr) {
    self.0.after(&new_sibling.0);
  }

  #[napi(js_name = "before")]
  pub fn before(&self, new_sibling: &NodeRepr) {
    self.0.before(&new_sibling.0);
  }

  #[napi(js_name = "insertBefore")]
  pub fn insert_before_node(
    &self,
    new_node: &NodeRepr,
    ref_node: Option<&NodeRepr>,
  ) -> napi::Result<NodeRepr> {
    self
      .0
      .insert_before_node(&new_node.0, ref_node.map(|n| &n.0))
      .map(NodeRepr)
      .map_err(|e| napi::Error::new(napi::Status::InvalidArg, e))
  }

  #[napi]
  pub fn remove(&self) {
    self.0.remove();
  }

  #[napi]
  pub fn set_attribute(&self, name: String, value: String) {
    self.0.set_attribute(name, value);
  }

  #[napi]
  pub fn remove_attribute(&self, name: String) {
    self.0.remove_attribute(name);
  }

  #[napi(js_name = "toggleAttribute")]
  pub fn toggle_attribute(&self, name: String, force: Option<bool>) -> bool {
    self.0.toggle_attribute(name, force)
  }

  #[napi(js_name = "setAttributeNS")]
  pub fn set_attribute_ns(&self, namespace: Option<String>, name: String, value: String) {
    self.0.set_attribute_ns(namespace, name, value);
  }

  #[napi(js_name = "removeAttributeNS")]
  pub fn remove_attribute_ns(&self, namespace: Option<String>, local_name: String) {
    self.0.remove_attribute_ns(namespace, local_name);
  }

  #[napi(js_name = "createElement")]
  pub fn create_element(&self, tag_name: String) -> NodeRepr {
    NodeRepr(self.0.create_element(tag_name))
  }

  #[napi(js_name = "createTextNode")]
  pub fn create_text_node(&self, data: String) -> NodeRepr {
    NodeRepr(self.0.create_text_node(data))
  }

  #[napi(js_name = "createComment")]
  pub fn create_comment(&self, data: String) -> NodeRepr {
    NodeRepr(self.0.create_comment(data))
  }

  #[napi(js_name = "createDocumentFragment")]
  pub fn create_document_fragment(&self) -> NodeRepr {
    NodeRepr(self.0.create_document_fragment())
  }

  #[napi(js_name = "createProcessingInstruction")]
  pub fn create_processing_instruction(&self, target: String, data: String) -> NodeRepr {
    NodeRepr(self.0.create_processing_instruction(target, data))
  }

  #[napi(js_name = "importNode")]
  pub fn import_node(&self, external_node: &NodeRepr, deep: Option<bool>) -> NodeRepr {
    NodeRepr(self.0.import_node(&external_node.0, deep))
  }

  #[napi(js_name = "adoptNode")]
  pub fn adopt_node(&self, external_node: &NodeRepr) -> NodeRepr {
    NodeRepr(self.0.adopt_node(&external_node.0))
  }

  #[napi(js_name = "replaceChild")]
  pub fn replace_child(
    &self,
    new_child: &NodeRepr,
    old_child: &NodeRepr,
  ) -> napi::Result<NodeRepr> {
    self
      .0
      .replace_child(&new_child.0, &old_child.0)
      .map(NodeRepr)
      .map_err(|e| napi::Error::new(napi::Status::InvalidArg, e))
  }

  #[napi(js_name = "replaceWith")]
  pub fn replace_with(&self, new_node: &NodeRepr) {
    self.0.replace_with(&new_node.0);
  }
}
