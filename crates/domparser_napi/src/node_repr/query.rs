use html5ever::serialize::{self, serialize, SerializeOpts};
use html5ever::{local_name, namespace_url, ns, LocalName, Namespace};
use kuchikiki::ExpandedName;

use crate::serializer::serialize_text_only;

use super::NodeRepr;

#[napi]
impl NodeRepr {
  /// Select the the fist node that match the given css selector, like document.querySelector.
  ///
  pub fn select(&self, selectors: String) -> Option<NodeRepr> {
    self.0.select_first(&selectors).ok().map(Into::into)
  }

  /// Select all nodes that match the given css selector, like document.querySelectorAll.
  ///
  pub fn select_all(&self, selectors: String) -> Vec<NodeRepr> {
    self
      .0
      .select(&selectors)
      .map_or(vec![], |els| els.map(Into::into).collect())
  }

  /// Get attribute value of this node by given name.
  ///
  #[napi]
  pub fn get_attribute(&self, name: String) -> Option<String> {
    self
      .0
      .as_element()
      .and_then(|e| e.attributes.borrow().get(name).map(|v| v.to_string()))
  }

  #[napi(js_name = "getAttributeNames")]
  pub fn get_attribute_names(&self) -> Vec<String> {
    self.0.as_element().map_or_else(Vec::new, |e| {
      e.attributes
        .borrow()
        .map
        .keys()
        .map(|expanded_name| expanded_name.local.to_string())
        .collect()
    })
  }

  #[napi(js_name = "hasAttributes")]
  pub fn has_attributes(&self) -> bool {
    self
      .0
      .as_element()
      .map_or(false, |e| !e.attributes.borrow().map.is_empty())
  }

  #[napi(js_name = "hasChildNodes")]
  pub fn has_child_nodes(&self) -> bool {
    self.0.children().next().is_some()
  }

  #[napi(js_name = "compareDocumentPosition")]
  pub fn compare_document_position(&self, other: &NodeRepr) -> u32 {
    if self.0 == other.0 {
      return 0;
    }

    let self_ancestors: Vec<_> = self.0.inclusive_ancestors().collect();
    let other_ancestors: Vec<_> = other.0.inclusive_ancestors().collect();

    let self_root = self_ancestors.last().unwrap();
    let other_root = other_ancestors.last().unwrap();

    if self_root != other_root {
      // Disconnected and implementation specific
      return 1 | 32;
    }

    // Check for containment
    if self_ancestors.contains(&other.0) {
      // other is ancestor of self -> other contains self -> self is contained by other
      // Relation: other contains self.
      // Return: DOCUMENT_POSITION_CONTAINS (8) | DOCUMENT_POSITION_PRECEDING (2)
      return 8 | 2;
    }

    if other_ancestors.contains(&self.0) {
      // self is ancestor of other -> self contains other -> other is contained by self
      // Relation: self contains other.
      // Return: DOCUMENT_POSITION_CONTAINED_BY (16) | DOCUMENT_POSITION_FOLLOWING (4)
      return 16 | 4;
    }

    // Find LCA
    let mut lca = None;
    // Iterate from root downwards (reverse of inclusive_ancestors which is bottom-up)
    for (a, b) in self_ancestors
      .iter()
      .rev()
      .zip(other_ancestors.iter().rev())
    {
      if a == b {
        lca = Some(a);
      } else {
        break;
      }
    }

    if let Some(lca_node) = lca {
      // Find the children of LCA that lead to self and other
      let mut self_child = self.0.clone();
      // We need to find the child of lca that is an ancestor of self (or self itself)
      // Since we have the ancestors list, we can just pick the one before lca.
      // self_ancestors is [self, parent, ..., lca, ..., root]
      // We want the node in self_ancestors that is just before lca.

      // Find index of lca in self_ancestors
      let self_lca_idx = self_ancestors.iter().position(|x| x == lca_node).unwrap();
      if self_lca_idx > 0 {
        self_child = self_ancestors[self_lca_idx - 1].clone();
      }

      let other_lca_idx = other_ancestors.iter().position(|x| x == lca_node).unwrap();
      let mut other_child = other.0.clone();
      if other_lca_idx > 0 {
        other_child = other_ancestors[other_lca_idx - 1].clone();
      }

      // Compare siblings order
      let mut current = self_child.next_sibling();
      while let Some(sibling) = current {
        if sibling == other_child {
          // other is after self
          return 4; // DOCUMENT_POSITION_FOLLOWING
        }
        current = sibling.next_sibling();
      }

      // If not found after, it must be before
      return 2; // DOCUMENT_POSITION_PRECEDING
    }

    1 // Should not happen if roots are same
  }

  /// Get the serialized html of this node, including its all descendants and itelf.
  ///
  pub fn outer_html(&self) -> String {
    let mut u8_vec = Vec::new();
    serialize(
      &mut u8_vec,
      self,
      SerializeOpts {
        traversal_scope: serialize::TraversalScope::IncludeNode,
        create_missing_parent: false,
        scripting_enabled: true,
      },
    )
    .unwrap();
    unsafe { String::from_utf8_unchecked(u8_vec) }
  }

  /// Get the serialized html of this node, only including its all descendants.
  ///
  pub fn inner_html(&self) -> String {
    let mut buf = Vec::<u8>::new();
    serialize(
      &mut buf,
      self,
      SerializeOpts {
        traversal_scope: serialize::TraversalScope::ChildrenOnly(None),
        create_missing_parent: false,
        scripting_enabled: true,
      },
    )
    .unwrap();
    unsafe { String::from_utf8_unchecked(buf) }
  }

  /// Get all text nodes content of this node, including its all descendants and itelf.
  ///
  pub fn text(&self) -> String {
    let mut buf = Vec::<u8>::new();
    serialize_text_only(&self.0, &mut buf).unwrap();
    unsafe { String::from_utf8_unchecked(buf) }
  }

  #[napi(js_name = "querySelector")]
  pub fn query_selector(&self, selectors: String) -> Option<NodeRepr> {
    self.select(selectors)
  }

  #[napi(js_name = "querySelectorAll")]
  pub fn query_selector_all(&self, selectors: String) -> Vec<NodeRepr> {
    self.select_all(selectors)
  }

  #[napi(js_name = "hasAttribute")]
  pub fn has_attribute(&self, name: String) -> bool {
    self.get_attribute(name).is_some()
  }

  #[napi(js_name = "getAttributeNS")]
  pub fn get_attribute_ns(&self, namespace: Option<String>, local_name: String) -> Option<String> {
    let ns = namespace.map(Into::into).unwrap_or(Namespace::from(""));
    let local = LocalName::from(local_name);
    let expanded = ExpandedName {
      ns: ns,
      local: local,
    };

    self.0.as_element().and_then(|e| {
      e.attributes
        .borrow()
        .map
        .get(&expanded)
        .map(|a| a.value.to_string())
    })
  }

  #[napi(js_name = "hasAttributeNS")]
  pub fn has_attribute_ns(&self, namespace: Option<String>, local_name: String) -> bool {
    let ns = namespace.map(Into::into).unwrap_or(Namespace::from(""));
    let local = LocalName::from(local_name);
    let expanded = ExpandedName {
      ns: ns,
      local: local,
    };

    self
      .0
      .as_element()
      .map_or(false, |e| e.attributes.borrow().map.contains_key(&expanded))
  }

  #[napi(js_name = "lookupPrefix")]
  pub fn lookup_prefix(&self, namespace: Option<String>) -> Option<String> {
    let ns_str = namespace.unwrap_or_default();
    if ns_str.is_empty() {
      return None;
    }

    let mut current = Some(self.0.clone());
    while let Some(node) = current {
      if let Some(ele) = node.as_element() {
        // Check attributes for xmlns:prefix="namespace"
        for (name, attr) in ele.attributes.borrow().map.iter() {
          if name.ns == ns!(xmlns) {
            if attr.value.to_string() == ns_str {
              return Some(name.local.to_string());
            }
          }
        }
      }
      current = node.parent();
    }
    None
  }

  #[napi(js_name = "lookupNamespaceURI")]
  pub fn lookup_namespace_uri(&self, prefix: Option<String>) -> Option<String> {
    let prefix_str = prefix.unwrap_or_default();

    let mut current = Some(self.0.clone());
    while let Some(node) = current {
      if let Some(ele) = node.as_element() {
        if prefix_str.is_empty() {
          // Look for default namespace xmlns="..."
          // In kuchikiki/html5ever, xmlns attribute has no prefix and local name "xmlns"
          // But wait, xmlns attribute is in xmlns namespace?
          // Actually xmlns="..." -> local: xmlns, ns: xmlns (http://www.w3.org/2000/xmlns/)
          // xmlns:p="..." -> local: p, ns: xmlns

          // Let's check how kuchikiki stores it.
          // Usually xmlns attributes are stored as attributes in the xmlns namespace.

          for (name, attr) in ele.attributes.borrow().map.iter() {
            if name.ns == ns!(xmlns) && name.local == local_name!("xmlns") {
              let val = attr.value.to_string();
              if !val.is_empty() {
                return Some(val);
              } else {
                return None; // Default namespace explicitly set to empty
              }
            }
          }
        } else {
          // Look for xmlns:prefix="..."
          for (name, attr) in ele.attributes.borrow().map.iter() {
            if name.ns == ns!(xmlns) && name.local.to_string() == prefix_str {
              let val = attr.value.to_string();
              if !val.is_empty() {
                return Some(val);
              } else {
                return None; // Namespace explicitly set to empty
              }
            }
          }
        }
      }
      current = node.parent();
    }
    None
  }

  #[napi(js_name = "isDefaultNamespace")]
  pub fn is_default_namespace(&self, namespace: Option<String>) -> bool {
    let ns = namespace.unwrap_or_default();
    if ns.is_empty() {
      // If namespace is empty, we check if default namespace is null/empty
      return self.lookup_namespace_uri(None).is_none();
    }
    self.lookup_namespace_uri(None).as_deref() == Some(&ns)
  }

  #[napi(js_name = "getElementById")]
  pub fn get_element_by_id(&self, id: String) -> Option<NodeRepr> {
    self.select(format!("#{}", id))
  }

  #[napi(js_name = "getElementsByClassName")]
  pub fn get_elements_by_class_name(&self, class_names: String) -> Vec<NodeRepr> {
    let selector = class_names
      .split_whitespace()
      .map(|c| format!(".{}", c))
      .collect::<Vec<_>>()
      .join("");
    if selector.is_empty() {
      vec![]
    } else {
      self.select_all(selector)
    }
  }

  #[napi(js_name = "getElementsByTagName")]
  pub fn get_elements_by_tag_name(&self, tag_name: String) -> Vec<NodeRepr> {
    self.select_all(tag_name)
  }

  #[napi]
  pub fn contains(&self, other_node: &NodeRepr) -> bool {
    if self.0 == other_node.0 {
      return true;
    }
    other_node.0.ancestors().any(|a| a == self.0)
  }

  #[napi(js_name = "isEqualNode")]
  pub fn is_equal_node(&self, other_node: &NodeRepr) -> bool {
    self.outer_html() == other_node.outer_html()
  }

  #[napi(getter)]
  pub fn head(&self) -> Option<NodeRepr> {
    self.select("head".to_string())
  }

  #[napi(getter)]
  pub fn body(&self) -> Option<NodeRepr> {
    self.select("body".to_string())
  }

  #[napi(getter)]
  pub fn title(&self) -> String {
    self
      .select("title".to_string())
      .map(|t| t.text())
      .unwrap_or_default()
  }

  #[napi(getter)]
  pub fn document_element(&self) -> Option<NodeRepr> {
    if let kuchikiki::NodeData::Document(_) = self.0.data() {
      self
        .0
        .children()
        .find(|n| n.as_element().is_some())
        .map(NodeRepr::from)
    } else {
      None
    }
  }

  #[napi]
  pub fn matches(&self, selectors: String) -> bool {
    if self.0.as_element().is_none() {
      return false;
    }
    if let Some(parent) = self.0.parent() {
      if let Ok(mut matches) = parent.select(&selectors) {
        return matches.any(|el| el.as_node() == &self.0);
      }
    }
    false
  }

  #[napi]
  pub fn closest(&self, selectors: String) -> Option<NodeRepr> {
    let mut current = Some(self.0.clone());
    while let Some(node) = current {
      if node.as_element().is_some() {
        let repr = NodeRepr::from(node.clone());
        if repr.matches(selectors.clone()) {
          return Some(repr);
        }
      }
      current = node.parent();
    }
    None
  }
}
