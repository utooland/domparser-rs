use super::NodeRepr;
use std::collections::HashMap;

#[napi]
impl NodeRepr {
  #[napi(getter)]
  pub fn node_type(&self) -> i32 {
    self.0.node_type()
  }

  #[napi(getter)]
  pub fn node_name(&self) -> String {
    self.0.node_name()
  }

  #[napi(getter)]
  pub fn tag_name(&self) -> Option<String> {
    self.0.tag_name()
  }

  #[napi(getter, js_name = "namespaceURI")]
  pub fn namespace_uri(&self) -> Option<String> {
    self.0.namespace_uri()
  }

  #[napi(getter)]
  pub fn prefix(&self) -> Option<String> {
    self.0.prefix()
  }

  #[napi(getter)]
  pub fn local_name(&self) -> Option<String> {
    self.0.local_name()
  }

  #[napi(getter)]
  pub fn id(&self) -> String {
    self.0.id()
  }

  #[napi(setter)]
  pub fn set_id(&self, id: String) {
    self.0.set_id(id);
  }

  #[napi(getter)]
  pub fn class_name(&self) -> String {
    self.0.class_name()
  }

  #[napi(setter)]
  pub fn set_class_name(&self, class_name: String) {
    self.0.set_class_name(class_name);
  }

  #[napi(getter)]
  pub fn parent_node(&self) -> Option<NodeRepr> {
    self.0.parent_node().map(NodeRepr)
  }

  #[napi(getter)]
  pub fn first_child(&self) -> Option<NodeRepr> {
    self.0.first_child().map(NodeRepr)
  }

  #[napi(getter)]
  pub fn last_child(&self) -> Option<NodeRepr> {
    self.0.last_child().map(NodeRepr)
  }

  #[napi(getter)]
  pub fn previous_sibling(&self) -> Option<NodeRepr> {
    self.0.previous_sibling().map(NodeRepr)
  }

  #[napi(getter)]
  pub fn next_sibling(&self) -> Option<NodeRepr> {
    self.0.next_sibling().map(NodeRepr)
  }

  #[napi(getter)]
  pub fn parent_element(&self) -> Option<NodeRepr> {
    self.0.parent_element().map(NodeRepr)
  }

  #[napi(getter)]
  pub fn first_element_child(&self) -> Option<NodeRepr> {
    self.0.first_element_child().map(NodeRepr)
  }

  #[napi(getter)]
  pub fn last_element_child(&self) -> Option<NodeRepr> {
    self.0.last_element_child().map(NodeRepr)
  }

  #[napi(getter)]
  pub fn previous_element_sibling(&self) -> Option<NodeRepr> {
    self.0.previous_element_sibling().map(NodeRepr)
  }

  #[napi(getter)]
  pub fn next_element_sibling(&self) -> Option<NodeRepr> {
    self.0.next_element_sibling().map(NodeRepr)
  }

  #[napi(getter)]
  pub fn children(&self) -> Vec<NodeRepr> {
    self.0.children().into_iter().map(NodeRepr).collect()
  }

  #[napi(getter)]
  pub fn child_element_count(&self) -> u32 {
    self.0.child_element_count()
  }

  #[napi(js_name = "getRootNode")]
  pub fn get_root_node(&self) -> NodeRepr {
    NodeRepr(self.0.get_root_node())
  }

  #[napi(getter)]
  pub fn node_value(&self) -> Option<String> {
    self.0.node_value()
  }

  #[napi(setter)]
  pub fn set_node_value(&self, value: Option<String>) {
    self.0.set_node_value(value);
  }

  #[napi(getter)]
  pub fn target(&self) -> Option<String> {
    self.0.target()
  }

  #[napi(getter)]
  pub fn name(&self) -> Option<String> {
    self.0.name()
  }

  #[napi(getter)]
  pub fn public_id(&self) -> Option<String> {
    self.0.public_id()
  }

  #[napi(getter)]
  pub fn system_id(&self) -> Option<String> {
    self.0.system_id()
  }

  #[napi(getter)]
  pub fn doctype(&self) -> Option<NodeRepr> {
    self.0.doctype().map(NodeRepr)
  }

  #[napi(getter)]
  pub fn data(&self) -> Option<String> {
    self.0.data()
  }

  #[napi(setter)]
  pub fn set_data(&self, value: String) {
    self.0.set_data(value);
  }

  #[napi(getter, js_name = "textContent")]
  pub fn text_content_getter(&self) -> String {
    self.0.text_content_getter()
  }

  #[napi(setter, js_name = "textContent")]
  pub fn set_text_content(&self, text: String) {
    self.0.set_text_content(text);
  }

  #[napi(js_name = "isSameNode")]
  pub fn is_same_node(&self, other_node: &NodeRepr) -> bool {
    self.0.is_same_node(&other_node.0)
  }

  #[napi(getter, js_name = "innerHTML")]
  pub fn inner_html_getter(&self) -> String {
    self.0.inner_html_getter()
  }

  #[napi(getter)]
  pub fn length(&self) -> u32 {
    self.0.length()
  }

  #[napi(setter, js_name = "innerHTML")]
  pub fn set_inner_html(&self, html: String) {
    self.0.set_inner_html(html);
  }

  #[napi(getter, js_name = "outerHTML")]
  pub fn outer_html_getter(&self) -> String {
    self.0.outer_html_getter()
  }

  #[napi(setter, js_name = "outerHTML")]
  pub fn set_outer_html(&self, html: String) {
    self.0.set_outer_html(html);
  }

  #[napi(getter)]
  pub fn owner_document(&self) -> Option<NodeRepr> {
    self.0.owner_document().map(NodeRepr)
  }

  #[napi(js_name = "toString")]
  pub fn to_string_js(&self) -> String {
    self.0.to_string_js()
  }

  #[napi(getter)]
  pub fn child_nodes(&self) -> Vec<NodeRepr> {
    self.0.child_nodes().into_iter().map(NodeRepr).collect()
  }

  #[napi(js_name = "_classListAdd")]
  pub fn _class_list_add(&self, token: String) {
    self.0._class_list_add(token);
  }

  #[napi(js_name = "_classListRemove")]
  pub fn _class_list_remove(&self, token: String) {
    self.0._class_list_remove(token);
  }

  #[napi(js_name = "_classListToggle")]
  pub fn _class_list_toggle(&self, token: String, force: Option<bool>) -> bool {
    self.0._class_list_toggle(token, force)
  }

  #[napi(js_name = "_classListContains")]
  pub fn _class_list_contains(&self, token: String) -> bool {
    self.0._class_list_contains(token)
  }

  #[napi(js_name = "_datasetGet")]
  pub fn _dataset_get(&self) -> HashMap<String, String> {
    self.0._dataset_get()
  }

  #[napi(js_name = "_datasetSet")]
  pub fn _dataset_set(&self, key: String, value: String) {
    self.0._dataset_set(key, value);
  }

  #[napi(js_name = "_datasetRemove")]
  pub fn _dataset_remove(&self, key: String) {
    self.0._dataset_remove(key);
  }

  #[napi(js_name = "substringData")]
  pub fn substring_data(&self, offset: u32, count: u32) -> String {
    self.0.substring_data(offset, count)
  }

  #[napi(js_name = "appendData")]
  pub fn append_data(&self, data: String) {
    self.0.append_data(data);
  }

  #[napi(js_name = "insertData")]
  pub fn insert_data(&self, offset: u32, data: String) {
    self.0.insert_data(offset, data);
  }

  #[napi(js_name = "deleteData")]
  pub fn delete_data(&self, offset: u32, count: u32) {
    self.0.delete_data(offset, count);
  }

  #[napi(js_name = "replaceData")]
  pub fn replace_data(&self, offset: u32, count: u32, data: String) {
    self.0.replace_data(offset, count, data);
  }

  #[napi(js_name = "splitText")]
  pub fn split_text(&self, offset: u32) -> Option<NodeRepr> {
    self.0.split_text(offset).map(NodeRepr)
  }

  #[napi(js_name = "insertAdjacentHTML")]
  pub fn insert_adjacent_html(&self, position: String, html: String) {
    self.0.insert_adjacent_html(position, html);
  }

  #[napi(js_name = "insertAdjacentText")]
  pub fn insert_adjacent_text(&self, position: String, text: String) {
    self.0.insert_adjacent_text(position, text);
  }

  #[napi(js_name = "insertAdjacentElement")]
  pub fn insert_adjacent_element(&self, position: String, element: &NodeRepr) {
    self.0.insert_adjacent_element(position, &element.0);
  }

  #[napi]
  pub fn normalize(&self) {
    self.0.normalize();
  }

  #[napi(js_name = "lookupNamespaceURI")]
  pub fn lookup_namespace_uri(&self, prefix: Option<String>) -> Option<String> {
    self.0.lookup_namespace_uri(prefix)
  }

  #[napi(js_name = "lookupPrefix")]
  pub fn lookup_prefix(&self, namespace: String) -> Option<String> {
    self.0.lookup_prefix(namespace)
  }

  #[napi(js_name = "compareDocumentPosition")]
  pub fn compare_document_position(&self, other: &NodeRepr) -> u32 {
    self.0.compare_document_position(&other.0)
  }
}
