#[macro_use]
extern crate napi_derive;

use domparser::parse as parse_core;
use node_repr::NodeRepr;

mod node_repr;

/// Parse string input to a html tree, return the root node.
///
#[napi]
pub fn parse(html: String) -> NodeRepr {
  NodeRepr(parse_core(html))
}
