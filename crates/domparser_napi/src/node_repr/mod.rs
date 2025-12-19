use domparser::DomNode;

mod modify;
mod properties;
mod query;

#[napi]
#[derive(Clone)]
pub struct NodeRepr(pub(crate) DomNode);

impl From<DomNode> for NodeRepr {
  fn from(node: DomNode) -> Self {
    Self(node)
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

  /// Clone this node to a new instance, not clone its descendants.
  ///
  #[napi]
  pub fn clone_self_only(&self) -> NodeRepr {
    NodeRepr(self.0.clone_self_only())
  }

  /// Clone this node to a new instance, including its all descendants.
  ///
  #[napi]
  pub fn clone_recursive(&self) -> NodeRepr {
    NodeRepr(self.0.clone_recursive())
  }

  #[napi(js_name = "cloneNode")]
  pub fn clone_node(&self, deep: Option<bool>) -> NodeRepr {
    NodeRepr(self.0.clone_node(deep))
  }
}
