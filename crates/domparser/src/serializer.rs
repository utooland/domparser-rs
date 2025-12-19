use markup5ever_rcdom::{Handle, NodeData};
use std::io::{Result, Write};

pub fn serialize_text_only<Wr: Write>(handle: &Handle, writer: &mut Wr) -> Result<()> {
  match &handle.data {
    NodeData::Text { contents } => {
      writer.write_all(contents.borrow().as_bytes())?;
      Ok(())
    }
    NodeData::Element { .. } | NodeData::Document => {
      for child in handle.children.borrow().iter() {
        serialize_text_only(child, writer)?
      }
      Ok(())
    }
    _ => Ok(()),
  }
}
