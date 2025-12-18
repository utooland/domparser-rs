use kuchikiki::{ElementData, NodeDataRef, NodeRef};

mod modify;
mod properties;
mod query;

#[napi]
pub struct NodeRepr(pub(crate) NodeRef);

impl From<NodeDataRef<ElementData>> for NodeRepr {
  fn from(element: NodeDataRef<ElementData>) -> Self {
    Self(element.as_node().clone())
  }
}

impl From<NodeRef> for NodeRepr {
  fn from(node_ref: NodeRef) -> Self {
    Self(node_ref)
  }
}

#[napi]
impl NodeRepr {
  /// The node object, cann't be instantiated in javascript. So call the constructor will throw an error.
  ///
  #[napi(constructor, ts_return_type = "void")]
  pub fn constructor() {
    unreachable!()
  }
}

// Constants cannot be directly on the class in napi-rs easily if not supported.
// We can export them as a separate object or just skip them for now if it causes issues.
// Or we can try to put them in a separate module but that creates a separate object in JS.
// Let's try to remove them for now to fix the build, and maybe add them as getters if needed, or just skip.
// The user asked for "more else", so maybe these constants are not critical if they break the build.
// But wait, I can implement them as getters on the prototype or static getters?
// napi-rs supports static properties.

#[napi]
impl NodeRepr {
  #[napi(getter)]
  pub fn element_node() -> u16 {
    1
  }
  #[napi(getter)]
  pub fn attribute_node() -> u16 {
    2
  }
  #[napi(getter)]
  pub fn text_node() -> u16 {
    3
  }
  #[napi(getter)]
  pub fn cdata_section_node() -> u16 {
    4
  }
  #[napi(getter)]
  pub fn entity_reference_node() -> u16 {
    5
  }
  #[napi(getter)]
  pub fn entity_node() -> u16 {
    6
  }
  #[napi(getter)]
  pub fn processing_instruction_node() -> u16 {
    7
  }
  #[napi(getter)]
  pub fn comment_node() -> u16 {
    8
  }
  #[napi(getter)]
  pub fn document_node() -> u16 {
    9
  }
  #[napi(getter)]
  pub fn document_type_node() -> u16 {
    10
  }
  #[napi(getter)]
  pub fn document_fragment_node() -> u16 {
    11
  }
  #[napi(getter)]
  pub fn notation_node() -> u16 {
    12
  }

  #[napi(getter)]
  pub fn document_position_disconnected() -> u16 {
    1
  }
  #[napi(getter)]
  pub fn document_position_preceding() -> u16 {
    2
  }
  #[napi(getter)]
  pub fn document_position_following() -> u16 {
    4
  }
  #[napi(getter)]
  pub fn document_position_contains() -> u16 {
    8
  }
  #[napi(getter)]
  pub fn document_position_contained_by() -> u16 {
    16
  }
  #[napi(getter)]
  pub fn document_position_implementation_specific() -> u16 {
    32
  }
}

#[napi]
impl NodeRepr {
  /// Clone this node to a new instance, not clone its descendants.
  ///
  pub fn clone_self_only(&self) -> NodeRepr {
    let new_node_ref = NodeRef::new(self.0.data().clone());
    NodeRepr::from(new_node_ref)
  }

  /// Clone this node to a new instance, including its all descendants.
  ///
  pub fn clone_recursive(&self) -> NodeRepr {
    NodeRepr::from(clone_node_ref_recursive(&self.0))
  }

  #[napi(js_name = "cloneNode")]
  pub fn clone_node(&self, deep: Option<bool>) -> NodeRepr {
    if deep.unwrap_or(false) {
      self.clone_recursive()
    } else {
      self.clone_self_only()
    }
  }
}

fn clone_node_ref_recursive(node_ref: &NodeRef) -> NodeRef {
  let new_node_ref = NodeRef::new(node_ref.data().clone());

  node_ref.children().for_each(|child| {
    let child_node_ref = clone_node_ref_recursive(&child);
    new_node_ref.append(child_node_ref);
  });

  new_node_ref
}
