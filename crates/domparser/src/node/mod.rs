use markup5ever_rcdom::{Handle, Node, NodeData};
use std::cell::RefCell;
use std::rc::Rc;

mod modify;
mod properties;
mod query;

pub(crate) fn get_parent(node: &Handle) -> Option<Handle> {
  let parent_weak = node.parent.take();
  let parent = parent_weak.as_ref().and_then(|w| w.upgrade());
  node.parent.set(parent_weak);
  parent
}

#[derive(Clone)]
pub struct DomNode(pub Handle);

impl From<Handle> for DomNode {
  fn from(handle: Handle) -> Self {
    Self(handle)
  }
}

impl DomNode {
  /// Clone this node to a new instance, not clone its descendants.
  ///
  pub fn clone_self_only(&self) -> DomNode {
    let new_node = Node::new(clone_node_data(&self.0.data));
    DomNode(new_node)
  }

  /// Clone this node to a new instance, including its all descendants.
  ///
  pub fn clone_recursive(&self) -> DomNode {
    DomNode(clone_handle_recursive(&self.0))
  }

  pub fn clone_node(&self, deep: Option<bool>) -> DomNode {
    if deep.unwrap_or(false) {
      self.clone_recursive()
    } else {
      self.clone_self_only()
    }
  }
}

fn clone_node_data(data: &NodeData) -> NodeData {
  match data {
    NodeData::Document => NodeData::Document,
    NodeData::Doctype {
      name,
      public_id,
      system_id,
    } => NodeData::Doctype {
      name: name.clone(),
      public_id: public_id.clone(),
      system_id: system_id.clone(),
    },
    NodeData::Text { contents } => NodeData::Text {
      contents: RefCell::new(contents.borrow().clone()),
    },
    NodeData::Comment { contents } => NodeData::Comment {
      contents: contents.clone(),
    },
    NodeData::Element {
      name,
      attrs,
      template_contents,
      mathml_annotation_xml_integration_point,
    } => {
      let new_attrs = attrs.borrow().clone();
      let new_template_contents = if let Some(tc) = template_contents.borrow().as_ref() {
        Some(clone_handle_recursive(tc))
      } else {
        None
      };
      NodeData::Element {
        name: name.clone(),
        attrs: RefCell::new(new_attrs),
        template_contents: RefCell::new(new_template_contents),
        mathml_annotation_xml_integration_point: *mathml_annotation_xml_integration_point,
      }
    }
    NodeData::ProcessingInstruction { target, contents } => NodeData::ProcessingInstruction {
      target: target.clone(),
      contents: contents.clone(),
    },
  }
}

fn clone_handle_recursive(handle: &Handle) -> Handle {
  let new_node = Node::new(clone_node_data(&handle.data));
  for child in handle.children.borrow().iter() {
    let new_child = clone_handle_recursive(child);
    new_node.children.borrow_mut().push(new_child.clone());
    new_child.parent.set(Some(Rc::downgrade(&new_node)));
  }
  new_node
}
