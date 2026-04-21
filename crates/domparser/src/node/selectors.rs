use crate::node::{get_parent, DomNode};
use cssparser::ToCss;
use markup5ever_rcdom::NodeData;
use precomputed_hash::PrecomputedHash;
use selectors::attr::{AttrSelectorOperation, CaseSensitivity, NamespaceConstraint};
use selectors::bloom::{BloomStorageU8, CountingBloomFilter};
use selectors::matching::{ElementSelectorFlags, MatchingContext};
use selectors::parser::{NonTSPseudoClass, PseudoElement, SelectorImpl};
use selectors::{Element, OpaqueElement};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct CssString(pub String);
impl From<&str> for CssString {
  fn from(s: &str) -> Self {
    CssString(s.to_owned())
  }
}
impl From<String> for CssString {
  fn from(s: String) -> Self {
    CssString(s)
  }
}
impl AsRef<str> for CssString {
  fn as_ref(&self) -> &str {
    &self.0
  }
}
impl std::borrow::Borrow<str> for CssString {
  fn borrow(&self) -> &str {
    &self.0
  }
}
impl PrecomputedHash for CssString {
  fn precomputed_hash(&self) -> u32 {
    0
  }
}
impl ToCss for CssString {
  fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
  where
    W: std::fmt::Write,
  {
    cssparser::serialize_identifier(&self.0, dest)
  }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PseudoClass {}
impl ToCss for PseudoClass {
  fn to_css<W>(&self, _dest: &mut W) -> std::fmt::Result
  where
    W: std::fmt::Write,
  {
    Ok(())
  }
}
impl NonTSPseudoClass for PseudoClass {
  type Impl = DomParserSelectors;
  fn is_active_or_hover(&self) -> bool {
    false
  }
  fn is_user_action_state(&self) -> bool {
    false
  }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PseudoElem {}
impl ToCss for PseudoElem {
  fn to_css<W>(&self, _dest: &mut W) -> std::fmt::Result
  where
    W: std::fmt::Write,
  {
    Ok(())
  }
}
impl PseudoElement for PseudoElem {
  type Impl = DomParserSelectors;
}

#[derive(Clone, Debug, PartialEq)]
pub struct DomParserSelectors;

impl SelectorImpl for DomParserSelectors {
  type AttrValue = CssString;
  type Identifier = CssString;
  type LocalName = CssString;
  type NamespaceUrl = CssString;
  type NamespacePrefix = CssString;
  type BorrowedNamespaceUrl = str;
  type BorrowedLocalName = str;
  type NonTSPseudoClass = PseudoClass;
  type PseudoElement = PseudoElem;
  type ExtraMatchingData<'a> = ();
}

impl Element for DomNode {
  type Impl = DomParserSelectors;
  fn opaque(&self) -> OpaqueElement {
    OpaqueElement::new(self.0.as_ref())
  }
  fn parent_element(&self) -> Option<Self> {
    let mut cur = get_parent(&self.0);
    while let Some(parent) = cur {
      if matches!(parent.data, NodeData::Element { .. }) {
        return Some(DomNode(parent));
      }
      cur = get_parent(&parent);
    }
    None
  }
  fn parent_node_is_shadow_root(&self) -> bool {
    false
  }
  fn containing_shadow_host(&self) -> Option<Self> {
    None
  }
  fn is_pseudo_element(&self) -> bool {
    false
  }
  fn prev_sibling_element(&self) -> Option<Self> {
    let parent = get_parent(&self.0)?;
    let children = parent.children.borrow();
    let idx = children.iter().position(|n| std::ptr::eq(&**n, &*self.0))?;
    for i in (0..idx).rev() {
      if matches!(children[i].data, NodeData::Element { .. }) {
        return Some(DomNode(children[i].clone()));
      }
    }
    None
  }
  fn next_sibling_element(&self) -> Option<Self> {
    let parent = get_parent(&self.0)?;
    let children = parent.children.borrow();
    let idx = children.iter().position(|n| std::ptr::eq(&**n, &*self.0))?;
    for i in (idx + 1)..children.len() {
      if matches!(children[i].data, NodeData::Element { .. }) {
        return Some(DomNode(children[i].clone()));
      }
    }
    None
  }
  fn first_element_child(&self) -> Option<Self> {
    self
      .0
      .children
      .borrow()
      .iter()
      .find(|n| matches!(n.data, NodeData::Element { .. }))
      .map(|n| DomNode(n.clone()))
  }
  fn is_html_element_in_html_document(&self) -> bool {
    true
  }
  fn has_local_name(&self, local_name: &str) -> bool {
    if let NodeData::Element { name, .. } = &self.0.data {
      name.local.as_ref() == local_name
    } else {
      false
    }
  }
  fn has_namespace(&self, namespace: &str) -> bool {
    if let NodeData::Element { name, .. } = &self.0.data {
      name.ns.as_ref() == namespace
    } else {
      false
    }
  }
  fn is_same_type(&self, other: &Self) -> bool {
    if let (NodeData::Element { name: n1, .. }, NodeData::Element { name: n2, .. }) =
      (&self.0.data, &other.0.data)
    {
      n1 == n2
    } else {
      false
    }
  }
  fn attr_matches(
    &self,
    namespace: &NamespaceConstraint<&CssString>,
    local_name: &CssString,
    operation: &AttrSelectorOperation<&CssString>,
  ) -> bool {
    if let NodeData::Element { attrs, .. } = &self.0.data {
      for attr in attrs.borrow().iter() {
        if attr.name.local.as_ref() != local_name.as_ref() {
          continue;
        }
        match namespace {
          NamespaceConstraint::Specific(ns) if attr.name.ns.as_ref() != ns.as_ref() => continue,
          _ => {}
        }
        if operation.eval_str(attr.value.as_ref()) {
          return true;
        }
      }
    }
    false
  }
  fn match_non_ts_pseudo_class(
    &self,
    _: &PseudoClass,
    _: &mut MatchingContext<'_, DomParserSelectors>,
  ) -> bool {
    false
  }
  fn match_pseudo_element(
    &self,
    _: &PseudoElem,
    _: &mut MatchingContext<'_, DomParserSelectors>,
  ) -> bool {
    false
  }
  fn apply_selector_flags(&self, _: ElementSelectorFlags) {}
  fn is_link(&self) -> bool {
    if let NodeData::Element { name, .. } = &self.0.data {
      name.local.as_ref() == "a" || name.local.as_ref() == "area" || name.local.as_ref() == "link"
    } else {
      false
    }
  }
  fn is_html_slot_element(&self) -> bool {
    false
  }
  fn has_id(&self, id: &CssString, _: CaseSensitivity) -> bool {
    if let NodeData::Element { attrs, .. } = &self.0.data {
      for attr in attrs.borrow().iter() {
        if attr.name.local.as_ref() == "id" {
          return attr.value.as_ref() == id.as_ref();
        }
      }
    }
    false
  }
  fn has_class(&self, class: &CssString, _: CaseSensitivity) -> bool {
    if let NodeData::Element { attrs, .. } = &self.0.data {
      for attr in attrs.borrow().iter() {
        if attr.name.local.as_ref() == "class" {
          return attr
            .value
            .as_ref()
            .split_whitespace()
            .any(|c| c == class.as_ref());
        }
      }
    }
    false
  }
  fn has_custom_state(&self, _: &CssString) -> bool {
    false
  }
  fn imported_part(&self, _: &CssString) -> Option<CssString> {
    None
  }
  fn is_part(&self, _: &CssString) -> bool {
    false
  }
  fn is_empty(&self) -> bool {
    self
      .0
      .children
      .borrow()
      .iter()
      .all(|n| !matches!(n.data, NodeData::Element { .. } | NodeData::Text { .. }))
  }
  fn is_root(&self) -> bool {
    self.parent_element().is_none()
  }
  fn add_element_unique_hashes(&self, _: &mut CountingBloomFilter<BloomStorageU8>) -> bool {
    false
  }
}

pub struct SelectorParser;
impl<'i> selectors::parser::Parser<'i> for SelectorParser {
  type Impl = DomParserSelectors;
  type Error = selectors::parser::SelectorParseErrorKind<'i>;
}

pub fn parse_selectors(selectors: &str) -> Option<selectors::SelectorList<DomParserSelectors>> {
  let mut input = cssparser::ParserInput::new(selectors);
  let mut parser = cssparser::Parser::new(&mut input);
  selectors::SelectorList::parse(
    &SelectorParser,
    &mut parser,
    selectors::parser::ParseRelative::No,
  )
  .ok()
}
