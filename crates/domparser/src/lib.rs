use html5ever::tendril::TendrilSink;
use html5ever::{local_name, namespace_url, ns, parse_document, QualName};
use markup5ever_rcdom::{Node, NodeData, RcDom};
use std::cell::RefCell;
use std::rc::Rc;

pub mod node;
pub mod serializer;

pub use markup5ever_rcdom;
pub use node::DomNode;

pub fn parse(html: String) -> DomNode {
  let dom = parse_document(RcDom::default(), Default::default())
    .from_utf8()
    .read_from(&mut html.as_bytes())
    .unwrap();

  {
    let mut children = dom.document.children.borrow_mut();
    let has_html = children.iter().any(|c| {
      if let NodeData::Element { name, .. } = &c.data {
        name.local.as_ref() == "html"
      } else {
        false
      }
    });

    if !has_html {
      let html_name = QualName::new(None, ns!(html), local_name!("html"));
      let html_node = Node::new(NodeData::Element {
        name: html_name,
        attrs: RefCell::new(vec![]),
        template_contents: RefCell::new(None),
        mathml_annotation_xml_integration_point: false,
      });

      for child in children.drain(..) {
        html_node.children.borrow_mut().push(child.clone());
        child.parent.set(Some(Rc::downgrade(&html_node)));
      }

      children.push(html_node.clone());
      html_node.parent.set(Some(Rc::downgrade(&dom.document)));
    }
  }

  DomNode(dom.document)
}
