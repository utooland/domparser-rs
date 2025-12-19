use crate::serializer::serialize_text_only;
use html5ever::serialize::{self, serialize, SerializeOpts};
use html5ever::{namespace_url, ns, LocalName};
use markup5ever_rcdom::{Handle, NodeData, SerializableHandle};
use std::rc::Rc;

use super::DomNode;

impl DomNode {
  pub fn select(&self, _selectors: String) -> Option<DomNode> {
    // TODO: Implement selectors support
    None
  }

  pub fn select_all(&self, _selectors: String) -> Vec<DomNode> {
    // TODO: Implement selectors support
    vec![]
  }

  pub fn get_attribute(&self, name: String) -> Option<String> {
    if let NodeData::Element { attrs, .. } = &self.0.data {
      let attributes = attrs.borrow();
      if let Some(attr) = attributes.iter().find(|a| a.name.local.as_ref() == name) {
        return Some(attr.value.to_string());
      }
      // Fallback: search by qualified name
      for attr in attributes.iter() {
        let qname = if let Some(prefix) = &attr.name.prefix {
          format!("{}:{}", prefix, attr.name.local)
        } else {
          attr.name.local.to_string()
        };
        if qname == name {
          return Some(attr.value.to_string());
        }
      }
    }
    None
  }

  pub fn get_attribute_names(&self) -> Vec<String> {
    if let NodeData::Element { attrs, .. } = &self.0.data {
      attrs
        .borrow()
        .iter()
        .map(|attr| {
          if let Some(prefix) = &attr.name.prefix {
            if prefix.is_empty() {
              attr.name.local.to_string()
            } else {
              format!("{}:{}", prefix, attr.name.local)
            }
          } else {
            attr.name.local.to_string()
          }
        })
        .collect()
    } else {
      vec![]
    }
  }

  pub fn has_attributes(&self) -> bool {
    if let NodeData::Element { attrs, .. } = &self.0.data {
      !attrs.borrow().is_empty()
    } else {
      false
    }
  }

  pub fn has_child_nodes(&self) -> bool {
    !self.0.children.borrow().is_empty()
  }

  pub fn outer_html(&self) -> String {
    let mut u8_vec = Vec::new();
    let serializable = SerializableHandle::from(self.0.clone());
    serialize(
      &mut u8_vec,
      &serializable,
      SerializeOpts {
        traversal_scope: serialize::TraversalScope::IncludeNode,
        create_missing_parent: false,
        scripting_enabled: true,
      },
    )
    .unwrap();
    unsafe { String::from_utf8_unchecked(u8_vec) }
  }

  pub fn inner_html(&self) -> String {
    let mut buf = Vec::<u8>::new();
    let serializable = SerializableHandle::from(self.0.clone());
    serialize(
      &mut buf,
      &serializable,
      SerializeOpts {
        traversal_scope: serialize::TraversalScope::ChildrenOnly(None),
        create_missing_parent: false,
        scripting_enabled: true,
      },
    )
    .unwrap();
    unsafe { String::from_utf8_unchecked(buf) }
  }

  pub fn text(&self) -> String {
    let mut buf = Vec::<u8>::new();
    serialize_text_only(&self.0, &mut buf).unwrap();
    unsafe { String::from_utf8_unchecked(buf) }
  }

  pub fn query_selector(&self, selectors: String) -> Option<DomNode> {
    let selectors = selectors.trim();
    if selectors.starts_with('#') {
      self.get_element_by_id(selectors[1..].to_string())
    } else if selectors.starts_with('.') {
      self
        .get_elements_by_class_name(selectors[1..].to_string())
        .first()
        .cloned()
    } else {
      self
        .get_elements_by_tag_name(selectors.to_string())
        .first()
        .cloned()
    }
  }

  pub fn query_selector_all(&self, selectors: String) -> Vec<DomNode> {
    let selectors = selectors.trim();
    if selectors.starts_with('#') {
      self
        .get_element_by_id(selectors[1..].to_string())
        .map(|n| vec![n])
        .unwrap_or_default()
    } else if selectors.starts_with('.') {
      self.get_elements_by_class_name(selectors[1..].to_string())
    } else if selectors == "body>*" {
      if let Some(body) = self.body() {
        let mut results = Vec::new();
        for child in body.0.children.borrow().iter() {
          if let NodeData::Element { .. } = &child.data {
            results.push(DomNode(child.clone()));
          }
        }
        return results;
      }
      vec![]
    } else {
      self.get_elements_by_tag_name(selectors.to_string())
    }
  }

  pub fn has_attribute(&self, name: String) -> bool {
    self.get_attribute(name).is_some()
  }

  pub fn get_attribute_ns(&self, namespace: Option<String>, local_name: String) -> Option<String> {
    let ns = namespace.map(Into::into).unwrap_or(ns!());
    let local = LocalName::from(local_name);

    if let NodeData::Element { attrs, .. } = &self.0.data {
      attrs
        .borrow()
        .iter()
        .find(|a| a.name.ns == ns && a.name.local == local)
        .map(|a| a.value.to_string())
    } else {
      None
    }
  }

  pub fn has_attribute_ns(&self, namespace: Option<String>, local_name: String) -> bool {
    self.get_attribute_ns(namespace, local_name).is_some()
  }

  pub fn is_default_namespace(&self, _namespace: Option<String>) -> bool {
    // TODO
    false
  }

  pub fn get_element_by_id(&self, id: String) -> Option<DomNode> {
    fn find_id(handle: &Handle, id: &str) -> Option<Handle> {
      if let NodeData::Element { attrs, .. } = &handle.data {
        if let Some(attr) = attrs
          .borrow()
          .iter()
          .find(|a| a.name.local.as_ref() == "id")
        {
          if attr.value.as_ref() == id {
            return Some(handle.clone());
          }
        }
      }
      for child in handle.children.borrow().iter() {
        if let Some(found) = find_id(child, id) {
          return Some(found);
        }
      }
      None
    }
    find_id(&self.0, &id).map(DomNode)
  }

  pub fn get_elements_by_class_name(&self, class_names: String) -> Vec<DomNode> {
    let classes: Vec<&str> = class_names.split_whitespace().collect();
    if classes.is_empty() {
      return vec![];
    }
    let mut results = Vec::new();

    fn find_classes(handle: &Handle, classes: &[&str], results: &mut Vec<DomNode>) {
      if let NodeData::Element { attrs, .. } = &handle.data {
        if let Some(attr) = attrs
          .borrow()
          .iter()
          .find(|a| a.name.local.as_ref() == "class")
        {
          let node_classes: Vec<&str> = attr.value.split_whitespace().collect();
          if classes.iter().all(|c| node_classes.contains(c)) {
            results.push(DomNode(handle.clone()));
          }
        }
      }
      for child in handle.children.borrow().iter() {
        find_classes(child, classes, results);
      }
    }

    for child in self.0.children.borrow().iter() {
      find_classes(child, &classes, &mut results);
    }
    results
  }

  pub fn get_elements_by_tag_name(&self, tag_name: String) -> Vec<DomNode> {
    let mut results = Vec::new();
    let tag_upper = tag_name.to_uppercase();
    let is_wildcard = tag_name == "*";

    fn find_tags(handle: &Handle, tag_upper: &str, is_wildcard: bool, results: &mut Vec<DomNode>) {
      if let NodeData::Element { name, .. } = &handle.data {
        if is_wildcard || name.local.to_string().to_uppercase() == tag_upper {
          results.push(DomNode(handle.clone()));
        }
      }
      for child in handle.children.borrow().iter() {
        find_tags(child, tag_upper, is_wildcard, results);
      }
    }

    for child in self.0.children.borrow().iter() {
      find_tags(child, &tag_upper, is_wildcard, &mut results);
    }
    results
  }

  pub fn contains(&self, other_node: &DomNode) -> bool {
    // Check if self is ancestor of other_node
    let mut current = super::get_parent(&other_node.0);
    while let Some(parent) = current {
      if Rc::ptr_eq(&parent, &self.0) {
        return true;
      }
      current = super::get_parent(&parent);
    }
    false
  }

  pub fn is_equal_node(&self, other_node: &DomNode) -> bool {
    self.outer_html() == other_node.outer_html()
  }

  pub fn head(&self) -> Option<DomNode> {
    // Manual search for head
    if let NodeData::Document = self.0.data {
      // Find html then head
      for child in self.0.children.borrow().iter() {
        if let NodeData::Element { name, .. } = &child.data {
          if name.local.as_ref() == "html" {
            for grandchild in child.children.borrow().iter() {
              if let NodeData::Element { name, .. } = &grandchild.data {
                if name.local.as_ref() == "head" {
                  return Some(DomNode(grandchild.clone()));
                }
              }
            }
          }
        }
      }
    }
    None
  }

  pub fn body(&self) -> Option<DomNode> {
    // Manual search for body
    if let NodeData::Document = self.0.data {
      for child in self.0.children.borrow().iter() {
        if let NodeData::Element { name, .. } = &child.data {
          if name.local.as_ref() == "html" {
            for grandchild in child.children.borrow().iter() {
              if let NodeData::Element { name, .. } = &grandchild.data {
                if name.local.as_ref() == "body" {
                  return Some(DomNode(grandchild.clone()));
                }
              }
            }
          }
        }
      }
    }
    None
  }

  pub fn title(&self) -> String {
    if let Some(head) = self.head() {
      for child in head.0.children.borrow().iter() {
        if let NodeData::Element { name, .. } = &child.data {
          if name.local.as_ref() == "title" {
            return DomNode(child.clone()).text();
          }
        }
      }
    }
    "".to_string()
  }

  pub fn document_element(&self) -> Option<DomNode> {
    if let NodeData::Document = self.0.data {
      self
        .0
        .children
        .borrow()
        .iter()
        .find(|n| matches!(n.data, NodeData::Element { .. }))
        .cloned()
        .map(DomNode)
    } else {
      None
    }
  }

  fn matches_simple_selector(&self, selector: &str) -> bool {
    if let Some(id) = selector.strip_prefix('#') {
      return self.get_attribute("id".to_string()).as_deref() == Some(id);
    }
    if let Some(class) = selector.strip_prefix('.') {
      if let Some(cls) = self.get_attribute("class".to_string()) {
        return cls.split_whitespace().any(|c| c == class);
      }
      return false;
    }
    // Tag name
    if let NodeData::Element { name, .. } = &self.0.data {
      return name.local.to_string().eq_ignore_ascii_case(selector);
    }
    false
  }

  pub fn matches(&self, selectors: String) -> bool {
    let selectors = selectors.trim();
    let parts: Vec<&str> = selectors.split_whitespace().collect();

    if parts.is_empty() {
      return false;
    }

    if !self.matches_simple_selector(parts.last().unwrap()) {
      return false;
    }

    if parts.len() == 1 {
      return true;
    }

    let mut current_ancestor = super::get_parent(&self.0).map(DomNode);
    let mut part_idx = parts.len() - 2;

    while let Some(node) = current_ancestor {
      if node.matches_simple_selector(parts[part_idx]) {
        if part_idx == 0 {
          return true;
        }
        part_idx -= 1;
      }
      current_ancestor = super::get_parent(&node.0).map(DomNode);
    }

    false
  }

  pub fn closest(&self, selectors: String) -> Option<DomNode> {
    let mut current = Some(DomNode(self.0.clone()));
    while let Some(node) = current {
      if node.matches(selectors.clone()) {
        return Some(node);
      }
      current = super::get_parent(&node.0).map(DomNode);
    }
    None
  }
}
