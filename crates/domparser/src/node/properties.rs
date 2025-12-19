use super::DomNode;
use html5ever::parse_fragment;
use html5ever::serialize::{serialize, SerializeOpts, TraversalScope};
use html5ever::tendril::TendrilSink;
use html5ever::{local_name, namespace_url, ns, QualName};
use markup5ever_rcdom::{Handle, Node, NodeData, RcDom, SerializableHandle};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

impl DomNode {
  pub fn node_type(&self) -> i32 {
    match &self.0.data {
      NodeData::Element { name, .. } => {
        if name.local.as_ref() == "#document-fragment" {
          11
        } else {
          1
        }
      }
      NodeData::Text { .. } => 3,
      NodeData::Comment { .. } => 8,
      NodeData::Document => 9,
      NodeData::Doctype { .. } => 10,
      NodeData::ProcessingInstruction { .. } => 7,
    }
  }

  pub fn node_name(&self) -> String {
    match &self.0.data {
      NodeData::Element { name, .. } => {
        if name.local.as_ref() == "#document-fragment" {
          "#document-fragment".to_string()
        } else {
          name.local.to_string().to_uppercase()
        }
      }
      NodeData::Text { .. } => "#text".to_string(),
      NodeData::Comment { .. } => "#comment".to_string(),
      NodeData::Document => "#document".to_string(),
      NodeData::Doctype { name, .. } => name.to_string(),
      NodeData::ProcessingInstruction { target, .. } => target.to_string(),
    }
  }

  pub fn tag_name(&self) -> Option<String> {
    match &self.0.data {
      NodeData::Element { name, .. } => Some(name.local.to_string().to_uppercase()),
      _ => None,
    }
  }

  pub fn namespace_uri(&self) -> Option<String> {
    match &self.0.data {
      NodeData::Element { name, .. } => Some(name.ns.to_string()),
      _ => None,
    }
  }

  pub fn prefix(&self) -> Option<String> {
    match &self.0.data {
      NodeData::Element { name, .. } => name.prefix.as_ref().map(|p| p.to_string()),
      _ => None,
    }
  }

  pub fn local_name(&self) -> Option<String> {
    match &self.0.data {
      NodeData::Element { name, .. } => Some(name.local.to_string()),
      _ => None,
    }
  }

  pub fn id(&self) -> String {
    self.get_attribute("id".to_string()).unwrap_or_default()
  }

  pub fn set_id(&self, id: String) {
    self.set_attribute("id".to_string(), id);
  }

  pub fn class_name(&self) -> String {
    self.get_attribute("class".to_string()).unwrap_or_default()
  }

  pub fn set_class_name(&self, class_name: String) {
    self.set_attribute("class".to_string(), class_name);
  }

  pub fn parent_node(&self) -> Option<DomNode> {
    super::get_parent(&self.0).map(DomNode)
  }

  pub fn first_child(&self) -> Option<DomNode> {
    self.0.children.borrow().first().cloned().map(DomNode)
  }

  pub fn last_child(&self) -> Option<DomNode> {
    self.0.children.borrow().last().cloned().map(DomNode)
  }

  pub fn previous_sibling(&self) -> Option<DomNode> {
    let parent = super::get_parent(&self.0)?;
    let children = parent.children.borrow();
    let pos = children.iter().position(|x| Rc::ptr_eq(x, &self.0))?;
    if pos > 0 {
      Some(DomNode(children[pos - 1].clone()))
    } else {
      None
    }
  }

  pub fn next_sibling(&self) -> Option<DomNode> {
    let parent = super::get_parent(&self.0)?;
    let children = parent.children.borrow();
    let pos = children.iter().position(|x| Rc::ptr_eq(x, &self.0))?;
    if pos + 1 < children.len() {
      Some(DomNode(children[pos + 1].clone()))
    } else {
      None
    }
  }

  pub fn parent_element(&self) -> Option<DomNode> {
    super::get_parent(&self.0).and_then(|n| {
      if let NodeData::Element { .. } = n.data {
        Some(DomNode(n))
      } else {
        None
      }
    })
  }

  pub fn first_element_child(&self) -> Option<DomNode> {
    self
      .0
      .children
      .borrow()
      .iter()
      .find(|n| matches!(n.data, NodeData::Element { .. }))
      .cloned()
      .map(DomNode)
  }

  pub fn last_element_child(&self) -> Option<DomNode> {
    self
      .0
      .children
      .borrow()
      .iter()
      .rev()
      .find(|n| matches!(n.data, NodeData::Element { .. }))
      .cloned()
      .map(DomNode)
  }

  pub fn previous_element_sibling(&self) -> Option<DomNode> {
    let mut current = self.previous_sibling();
    while let Some(node) = current {
      if matches!(node.0.data, NodeData::Element { .. }) {
        return Some(node);
      }
      current = node.previous_sibling();
    }
    None
  }

  pub fn next_element_sibling(&self) -> Option<DomNode> {
    let mut current = self.next_sibling();
    while let Some(node) = current {
      if matches!(node.0.data, NodeData::Element { .. }) {
        return Some(node);
      }
      current = node.next_sibling();
    }
    None
  }

  pub fn children(&self) -> Vec<DomNode> {
    self
      .0
      .children
      .borrow()
      .iter()
      .filter(|n| matches!(n.data, NodeData::Element { .. }))
      .cloned()
      .map(DomNode)
      .collect()
  }

  pub fn child_element_count(&self) -> u32 {
    self
      .0
      .children
      .borrow()
      .iter()
      .filter(|n| matches!(n.data, NodeData::Element { .. }))
      .count() as u32
  }

  pub fn get_root_node(&self) -> DomNode {
    let mut current = self.0.clone();
    loop {
      let parent = super::get_parent(&current);
      if let Some(p) = parent {
        current = p;
      } else {
        break;
      }
    }
    DomNode(current)
  }

  pub fn node_value(&self) -> Option<String> {
    match &self.0.data {
      NodeData::Text { contents } => Some(contents.borrow().to_string()),
      NodeData::Comment { contents } => Some(contents.to_string()),
      NodeData::ProcessingInstruction { contents, .. } => Some(contents.to_string()),
      _ => None,
    }
  }

  pub fn set_node_value(&self, value: Option<String>) {
    if let Some(val) = value {
      match &self.0.data {
        NodeData::Text { contents } => {
          *contents.borrow_mut() = val.into();
        }
        _ => {}
      }
    }
  }

  pub fn target(&self) -> Option<String> {
    match &self.0.data {
      NodeData::ProcessingInstruction { target, .. } => Some(target.to_string()),
      _ => None,
    }
  }

  pub fn name(&self) -> Option<String> {
    match &self.0.data {
      NodeData::Doctype { name, .. } => Some(name.to_string()),
      _ => None,
    }
  }

  pub fn public_id(&self) -> Option<String> {
    match &self.0.data {
      NodeData::Doctype { public_id, .. } => Some(public_id.to_string()),
      _ => None,
    }
  }

  pub fn system_id(&self) -> Option<String> {
    match &self.0.data {
      NodeData::Doctype { system_id, .. } => Some(system_id.to_string()),
      _ => None,
    }
  }

  pub fn doctype(&self) -> Option<DomNode> {
    if let NodeData::Document = self.0.data {
      self
        .0
        .children
        .borrow()
        .iter()
        .find(|n| matches!(n.data, NodeData::Doctype { .. }))
        .cloned()
        .map(DomNode)
    } else {
      None
    }
  }

  pub fn data(&self) -> Option<String> {
    self.node_value()
  }

  pub fn set_data(&self, value: String) {
    self.set_node_value(Some(value));
  }

  pub fn text_content_getter(&self) -> String {
    fn get_text(handle: &Handle) -> String {
      match &handle.data {
        NodeData::Text { contents } => contents.borrow().to_string(),
        NodeData::Element { .. } | NodeData::Document => handle
          .children
          .borrow()
          .iter()
          .map(|child| get_text(child))
          .collect(),
        _ => "".to_string(),
      }
    }
    get_text(&self.0)
  }

  pub fn set_text_content(&self, text: String) {
    self.0.children.borrow_mut().clear();
    let text_node = Node::new(NodeData::Text {
      contents: RefCell::new(text.into()),
    });
    let text_handle = text_node; // Node::new returns Handle
    self.0.children.borrow_mut().push(text_handle.clone());
    text_handle.parent.set(Some(Rc::downgrade(&self.0)));
  }

  pub fn is_same_node(&self, other_node: &DomNode) -> bool {
    Rc::ptr_eq(&self.0, &other_node.0)
  }

  pub fn inner_html_getter(&self) -> String {
    let mut bytes = Vec::new();
    let serializable = SerializableHandle::from(self.0.clone());
    serialize(
      &mut bytes,
      &serializable,
      SerializeOpts {
        scripting_enabled: false,
        traversal_scope: TraversalScope::ChildrenOnly(None),
        create_missing_parent: false,
      },
    )
    .unwrap();
    String::from_utf8(bytes).unwrap()
  }

  pub fn length(&self) -> u32 {
    if let NodeData::Text { contents } = &self.0.data {
      contents.borrow().len() as u32
    } else {
      0
    }
  }

  pub fn set_inner_html(&self, html: String) {
    self.0.children.borrow_mut().clear();

    let context_name = match &self.0.data {
      NodeData::Element { name, .. } => name.clone(),
      _ => QualName::new(None, ns!(html), local_name!("body")),
    };

    let dom = parse_fragment(
      RcDom::default(),
      Default::default(),
      context_name.clone(),
      vec![],
    )
    .from_utf8()
    .read_from(&mut html.as_bytes())
    .unwrap();

    let mut new_children = dom.document.children.borrow_mut();

    // If the fragment parsed into a full html structure (html -> body -> ...), unwrap it.
    // This happens if the context requires it or if html5ever forces it.
    // We want the content that was inside the context.

    // Heuristic: if result is <html><head></head><body>...</body></html>, we probably want the content of body (if context was body).
    // But if context was div, we want content of div.

    // Let's try to flatten: if child is html, take its children.
    // If child is head or body, and context matches, take its children?

    let mut nodes_to_append = Vec::new();

    for child in new_children.drain(..) {
      nodes_to_append.push(child);
    }

    // Unwrap html if it's the only child
    if nodes_to_append.len() == 1 {
      if let NodeData::Element { name, .. } = &nodes_to_append[0].data {
        if name.local.as_ref() == "html" {
          let html_node = nodes_to_append.pop().unwrap();
          let mut html_children = html_node.children.borrow_mut();
          for child in html_children.drain(..) {
            nodes_to_append.push(child);
          }
        }
      }
    }

    // Unwrap body if context is body and we have body
    if context_name.local.as_ref() == "body" {
      // If we have head and body, we probably want body's children?
      // Or if we just have body.
      // If we have head (empty) and body.

      // Let's look for body element in nodes_to_append
      let body_index = nodes_to_append.iter().position(|n| {
        if let NodeData::Element { name, .. } = &n.data {
          name.local.as_ref() == "body"
        } else {
          false
        }
      });

      if let Some(idx) = body_index {
        let body_node = nodes_to_append.remove(idx);
        // We discard other nodes (like head) if we found body?
        // Or we keep them?
        // Usually innerHTML on body replaces body content.
        // If I parse "<span>foo</span>", I get html->head, body->span.
        // I want span.
        // So I should take children of body.

        let mut body_children = body_node.children.borrow_mut();
        nodes_to_append.clear(); // Discard head etc.
        for child in body_children.drain(..) {
          nodes_to_append.push(child);
        }
      }
    }

    for child in nodes_to_append {
      self.0.children.borrow_mut().push(child.clone());
      child.parent.set(Some(Rc::downgrade(&self.0)));
    }
  }

  pub fn outer_html_getter(&self) -> String {
    let mut bytes = Vec::new();
    let serializable = SerializableHandle::from(self.0.clone());
    let traversal_scope = if let NodeData::Document = self.0.data {
      TraversalScope::ChildrenOnly(None)
    } else {
      TraversalScope::IncludeNode
    };
    serialize(
      &mut bytes,
      &serializable,
      SerializeOpts {
        scripting_enabled: false,
        traversal_scope,
        create_missing_parent: false,
      },
    )
    .unwrap();
    String::from_utf8(bytes).unwrap()
  }

  pub fn set_outer_html(&self, html: String) {
    if let Some(parent) = super::get_parent(&self.0) {
      let context_name = match &parent.data {
        NodeData::Element { name, .. } => name.clone(),
        _ => QualName::new(None, ns!(html), local_name!("body")),
      };

      let dom = parse_fragment(RcDom::default(), Default::default(), context_name, vec![])
        .from_utf8()
        .read_from(&mut html.as_bytes())
        .unwrap();

      let mut parent_children = parent.children.borrow_mut();
      if let Some(pos) = parent_children.iter().position(|x| Rc::ptr_eq(x, &self.0)) {
        parent_children.remove(pos);
        let mut new_children = dom.document.children.borrow_mut();
        for (i, child) in new_children.drain(..).enumerate() {
          parent_children.insert(pos + i, child.clone());
          child.parent.set(Some(Rc::downgrade(&parent)));
        }
      }
    }
  }

  pub fn owner_document(&self) -> Option<DomNode> {
    let root = self.get_root_node();
    if matches!(root.0.data, NodeData::Document) {
      Some(root)
    } else {
      None
    }
  }

  pub fn to_string_js(&self) -> String {
    self.outer_html_getter()
  }

  pub fn child_nodes(&self) -> Vec<DomNode> {
    self
      .0
      .children
      .borrow()
      .iter()
      .cloned()
      .map(DomNode)
      .collect()
  }

  pub fn _class_list_add(&self, token: String) {
    let class = self.class_name();
    let mut classes: Vec<String> = class.split_whitespace().map(|s| s.to_string()).collect();
    if !classes.contains(&token) {
      classes.push(token);
      self.set_class_name(classes.join(" "));
    }
  }

  pub fn _class_list_remove(&self, token: String) {
    let class = self.class_name();
    let classes: Vec<String> = class
      .split_whitespace()
      .filter(|&s| s != token)
      .map(|s| s.to_string())
      .collect();
    self.set_class_name(classes.join(" "));
  }

  pub fn _class_list_toggle(&self, token: String, force: Option<bool>) -> bool {
    let class = self.class_name();
    let mut classes: Vec<String> = class.split_whitespace().map(|s| s.to_string()).collect();
    let contains = classes.contains(&token);

    let should_add = if let Some(f) = force { f } else { !contains };

    if should_add && !contains {
      classes.push(token);
      self.set_class_name(classes.join(" "));
      true
    } else if !should_add && contains {
      let new_classes: Vec<String> = classes.into_iter().filter(|s| s != &token).collect();
      self.set_class_name(new_classes.join(" "));
      false
    } else {
      contains
    }
  }

  pub fn _class_list_contains(&self, token: String) -> bool {
    let class = self.class_name();
    class.split_whitespace().any(|s| s == token)
  }

  pub fn _dataset_get(&self) -> HashMap<String, String> {
    let mut map = HashMap::new();
    if let NodeData::Element { attrs, .. } = &self.0.data {
      for attr in attrs.borrow().iter() {
        let name = attr.name.local.to_string();
        if name.starts_with("data-") {
          let key = kebab_to_camel(&name[5..]);
          map.insert(key, attr.value.to_string());
        }
      }
    }
    map
  }

  pub fn _dataset_set(&self, key: String, value: String) {
    let attr_name = format!("data-{}", camel_to_kebab(&key));
    self.set_attribute(attr_name, value);
  }

  pub fn _dataset_remove(&self, key: String) {
    let attr_name = format!("data-{}", camel_to_kebab(&key));
    self.remove_attribute(attr_name);
  }

  pub fn substring_data(&self, offset: u32, count: u32) -> String {
    if let Some(text) = self.node_value() {
      let chars: Vec<char> = text.chars().collect();
      let start = offset as usize;
      let len = count as usize;
      if start >= chars.len() {
        return "".to_string();
      }
      let end = std::cmp::min(start + len, chars.len());
      chars[start..end].iter().collect()
    } else {
      "".to_string()
    }
  }

  pub fn append_data(&self, data: String) {
    if let Some(text) = self.node_value() {
      self.set_node_value(Some(text + &data));
    }
  }

  pub fn insert_data(&self, offset: u32, data: String) {
    if let Some(text) = self.node_value() {
      let mut chars: Vec<char> = text.chars().collect();
      let start = std::cmp::min(offset as usize, chars.len());
      let data_chars: Vec<char> = data.chars().collect();
      chars.splice(start..start, data_chars);
      self.set_node_value(Some(chars.into_iter().collect()));
    }
  }

  pub fn delete_data(&self, offset: u32, count: u32) {
    if let Some(text) = self.node_value() {
      let mut chars: Vec<char> = text.chars().collect();
      let start = offset as usize;
      let len = count as usize;
      if start < chars.len() {
        let end = std::cmp::min(start + len, chars.len());
        chars.drain(start..end);
        self.set_node_value(Some(chars.into_iter().collect()));
      }
    }
  }

  pub fn replace_data(&self, offset: u32, count: u32, data: String) {
    self.delete_data(offset, count);
    self.insert_data(offset, data);
  }

  pub fn split_text(&self, offset: u32) -> Option<DomNode> {
    if let NodeData::Text { contents } = &self.0.data {
      let text = contents.borrow().to_string();
      let chars: Vec<char> = text.chars().collect();
      let split_idx = std::cmp::min(offset as usize, chars.len());

      let first_part: String = chars[..split_idx].iter().collect();
      let second_part: String = chars[split_idx..].iter().collect();

      *contents.borrow_mut() = first_part.into();

      let new_node = Node::new(NodeData::Text {
        contents: RefCell::new(second_part.into()),
      });

      if let Some(parent) = super::get_parent(&self.0) {
        let mut children = parent.children.borrow_mut();
        if let Some(pos) = children.iter().position(|x| Rc::ptr_eq(x, &self.0)) {
          children.insert(pos + 1, new_node.clone());
          new_node.parent.set(Some(Rc::downgrade(&parent)));
        }
      }

      Some(DomNode(new_node))
    } else {
      None
    }
  }

  pub fn insert_adjacent_html(&self, position: String, html: String) {
    let context_name = match &self.0.data {
      NodeData::Element { name, .. } => name.clone(),
      _ => QualName::new(None, ns!(html), local_name!("body")),
    };

    let dom = parse_fragment(
      RcDom::default(),
      Default::default(),
      context_name.clone(),
      vec![],
    )
    .from_utf8()
    .read_from(&mut html.as_bytes())
    .unwrap();

    let mut new_children = dom.document.children.borrow_mut();
    let mut nodes_to_append = Vec::new();

    for child in new_children.drain(..) {
      nodes_to_append.push(child);
    }

    if nodes_to_append.len() == 1 {
      if let NodeData::Element { name, .. } = &nodes_to_append[0].data {
        if name.local.as_ref() == "html" {
          let html_node = nodes_to_append.pop().unwrap();
          let mut html_children = html_node.children.borrow_mut();
          for child in html_children.drain(..) {
            nodes_to_append.push(child);
          }
        }
      }
    }

    if context_name.local.as_ref() == "body" {
      let body_index = nodes_to_append.iter().position(|n| {
        if let NodeData::Element { name, .. } = &n.data {
          name.local.as_ref() == "body"
        } else {
          false
        }
      });

      if let Some(idx) = body_index {
        let body_node = nodes_to_append.remove(idx);
        let mut body_children = body_node.children.borrow_mut();
        nodes_to_append.clear();
        for child in body_children.drain(..) {
          nodes_to_append.push(child);
        }
      }
    }

    self.insert_adjacent_nodes(&position, nodes_to_append);
  }

  pub fn insert_adjacent_text(&self, position: String, text: String) {
    let text_node = Node::new(NodeData::Text {
      contents: RefCell::new(text.into()),
    });
    self.insert_adjacent_nodes(&position, vec![text_node]);
  }

  pub fn insert_adjacent_element(&self, position: String, element: &DomNode) {
    self.insert_adjacent_nodes(&position, vec![element.0.clone()]);
  }

  fn insert_adjacent_nodes(&self, position: &str, nodes: Vec<Handle>) {
    match position.to_lowercase().as_str() {
      "beforebegin" => {
        if let Some(parent) = super::get_parent(&self.0) {
          let mut children = parent.children.borrow_mut();
          if let Some(pos) = children.iter().position(|x| Rc::ptr_eq(x, &self.0)) {
            for (i, node) in nodes.into_iter().enumerate() {
              children.insert(pos + i, node.clone());
              node.parent.set(Some(Rc::downgrade(&parent)));
            }
          }
        }
      }
      "afterbegin" => {
        let mut children = self.0.children.borrow_mut();
        for (i, node) in nodes.into_iter().enumerate() {
          children.insert(i, node.clone());
          node.parent.set(Some(Rc::downgrade(&self.0)));
        }
      }
      "beforeend" => {
        let mut children = self.0.children.borrow_mut();
        for node in nodes {
          children.push(node.clone());
          node.parent.set(Some(Rc::downgrade(&self.0)));
        }
      }
      "afterend" => {
        if let Some(parent) = super::get_parent(&self.0) {
          let mut children = parent.children.borrow_mut();
          if let Some(pos) = children.iter().position(|x| Rc::ptr_eq(x, &self.0)) {
            for (i, node) in nodes.into_iter().enumerate() {
              children.insert(pos + 1 + i, node.clone());
              node.parent.set(Some(Rc::downgrade(&parent)));
            }
          }
        }
      }
      _ => {}
    }
  }

  pub fn normalize(&self) {
    let mut children = self.0.children.borrow_mut();
    let mut i = 0;
    while i < children.len() {
      let is_text = matches!(children[i].data, NodeData::Text { .. });
      if is_text {
        if i + 1 < children.len() {
          if let NodeData::Text {
            contents: next_contents,
          } = &children[i + 1].data
          {
            if let NodeData::Text { contents } = &children[i].data {
              let mut s = contents.borrow_mut();
              s.push_slice(&next_contents.borrow());
            }
            children.remove(i + 1);
            continue;
          }
        }

        let is_empty = if let NodeData::Text { contents } = &children[i].data {
          contents.borrow().len() == 0
        } else {
          false
        };

        if is_empty {
          children.remove(i);
          continue;
        }
      } else {
        let child_handle = children[i].clone();
        drop(children);
        DomNode(child_handle).normalize();
        children = self.0.children.borrow_mut();
      }
      i += 1;
    }
  }

  pub fn lookup_namespace_uri(&self, prefix: Option<String>) -> Option<String> {
    let mut current = Some(self.0.clone());
    while let Some(node) = current {
      if let NodeData::Element { attrs, .. } = &node.data {
        for attr in attrs.borrow().iter() {
          if let Some(p) = &prefix {
            if attr.name.prefix.as_ref().map(|s| s.as_ref()) == Some("xmlns")
              && attr.name.local.as_ref() == p
            {
              return Some(attr.value.to_string());
            }
          } else {
            if attr.name.local.as_ref() == "xmlns" && attr.name.prefix.is_none() {
              return Some(attr.value.to_string());
            }
          }
        }
      }
      current = super::get_parent(&node);
    }
    None
  }

  pub fn lookup_prefix(&self, namespace: String) -> Option<String> {
    let mut current = Some(self.0.clone());
    while let Some(node) = current {
      if let NodeData::Element { attrs, .. } = &node.data {
        for attr in attrs.borrow().iter() {
          if attr.value.as_ref() == namespace {
            if attr.name.prefix.as_ref().map(|s| s.as_ref()) == Some("xmlns") {
              return Some(attr.name.local.to_string());
            }
          }
        }
      }
      current = super::get_parent(&node);
    }
    None
  }

  pub fn compare_document_position(&self, other: &DomNode) -> u32 {
    if Rc::ptr_eq(&self.0, &other.0) {
      return 0;
    }

    let root1 = self.get_root_node();
    let root2 = other.get_root_node();
    if !Rc::ptr_eq(&root1.0, &root2.0) {
      return 1 | 32;
    }

    let mut current = other.0.clone();
    while let Some(parent) = super::get_parent(&current) {
      if Rc::ptr_eq(&parent, &self.0) {
        return 20;
      }
      current = parent;
    }

    current = self.0.clone();
    while let Some(parent) = super::get_parent(&current) {
      if Rc::ptr_eq(&parent, &other.0) {
        return 10;
      }
      current = parent;
    }

    let mut path1 = vec![];
    let mut curr = self.0.clone();
    path1.push(curr.clone());
    while let Some(p) = super::get_parent(&curr) {
      path1.push(p.clone());
      curr = p;
    }
    path1.reverse();

    let mut path2 = vec![];
    curr = other.0.clone();
    path2.push(curr.clone());
    while let Some(p) = super::get_parent(&curr) {
      path2.push(p.clone());
      curr = p;
    }
    path2.reverse();

    let mut i = 0;
    while i < path1.len() && i < path2.len() && Rc::ptr_eq(&path1[i], &path2[i]) {
      i += 1;
    }

    if i == 0 {
      return 1 | 32;
    }

    let lca = &path1[i - 1];
    let child1 = &path1[i];
    let child2 = &path2[i];

    let children = lca.children.borrow();
    let pos1 = children.iter().position(|x| Rc::ptr_eq(x, child1)).unwrap();
    let pos2 = children.iter().position(|x| Rc::ptr_eq(x, child2)).unwrap();

    if pos1 < pos2 {
      return 4;
    } else {
      return 2;
    }
  }
}

fn kebab_to_camel(s: &str) -> String {
  let mut result = String::new();
  let mut next_upper = false;
  for c in s.chars() {
    if c == '-' {
      next_upper = true;
    } else {
      if next_upper {
        result.push(c.to_ascii_uppercase());
        next_upper = false;
      } else {
        result.push(c);
      }
    }
  }
  result
}

fn camel_to_kebab(s: &str) -> String {
  let mut result = String::new();
  for (i, c) in s.chars().enumerate() {
    if c.is_uppercase() {
      if i > 0 {
        result.push('-');
      }
      result.push(c.to_ascii_lowercase());
    } else {
      result.push(c);
    }
  }
  result
}
