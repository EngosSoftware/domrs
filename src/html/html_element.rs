use crate::{HtmlAttribute, DEFAULT_HTML_INDENT};
use std::fmt;
use std::fmt::Display;
use std::fmt::Write;

#[derive(Debug, Clone)]
pub struct HtmlElement {
  name: String,
  attributes: Vec<HtmlAttribute>,
  content: Option<String>,
  children: Vec<HtmlElement>,
  no_closing: bool,
  no_indent: bool,
}

impl Display for HtmlElement {
  /// Converts `HTML` element into text.
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut buffer = String::new();
    self.write(0, DEFAULT_HTML_INDENT, &mut buffer);
    write!(f, "{}", buffer)
  }
}

impl HtmlElement {
  /// Creates a new HTML element with specified name.
  pub fn new(name: &str) -> Self {
    Self {
      name: name.to_string(),
      attributes: vec![],
      content: None,
      children: vec![],
      no_closing: false,
      no_indent: false,
    }
  }

  pub(crate) fn no_indent(mut self) -> Self {
    self.no_indent = true;
    self
  }

  pub(crate) fn no_closing(mut self) -> Self {
    self.no_closing = true;
    self
  }

  /// Creates a new `div` element with optional class.
  pub fn new_div(class: Option<&str>) -> Self {
    let mut element = Self::new("div");
    if let Some(class_name) = class {
      element.set_class(class_name);
    }
    element
  }

  /// Sets an attribute of the HTML element.
  pub fn set_attr<T: ToString>(&mut self, name: &str, value: T) {
    self.attributes.push(HtmlAttribute {
      name: name.to_string(),
      value: value.to_string(),
    })
  }

  /// Sets a `class` attribute of the HTML element.
  pub fn set_class(&mut self, class: &str) {
    self.attributes.push(HtmlAttribute {
      name: "class".to_string(),
      value: class.to_string(),
    })
  }

  /// Sets a `style` attribute of the HTML element.
  pub fn set_style(&mut self, style: &str) {
    self.attributes.push(HtmlAttribute {
      name: "style".to_string(),
      value: style.to_string(),
    })
  }

  /// Adds a child element.
  pub fn add_child(&mut self, e: HtmlElement) {
    self.children.push(e);
  }

  /// Adds an optional child element.
  pub fn add_child_opt(&mut self, e: Option<HtmlElement>) {
    if let Some(element) = e {
      self.children.push(element);
    }
  }

  /// Adds multiple children elements.
  pub fn add_children(&mut self, elements: Vec<HtmlElement>) {
    for element in elements {
      self.children.push(element);
    }
  }

  /// Sets the content of the HTML element.
  pub fn set_content(&mut self, content: &str) {
    self.content = Some(content.to_string());
  }

  /// Serializes the element to its textual representation.
  pub fn write(&self, mut indentation: usize, indent: usize, buffer: &mut String) {
    if self.no_indent && indentation >= indent {
      indentation -= indent;
    }
    let _ = write!(buffer, "{}<{}", get_indentation(self.no_indent, indentation), self.name);
    for attribute in &self.attributes {
      let _ = write!(buffer, r#" {}="{}""#, attribute.name, attribute.value);
    }
    if self.children.is_empty() {
      if let Some(content) = &self.content {
        let line_count = content.lines().count();
        if line_count > 1 {
          let _ = write!(buffer, ">");
          for line in content.lines() {
            let _ = write!(buffer, "\n{}{}", get_indentation(false, indentation + indent), line);
          }
          let _ = write!(buffer, "\n{}</{}>", get_indentation(false, indentation), self.name);
        } else {
          let _ = write!(buffer, ">{}</{}>", content, self.name);
        }
      } else {
        let _ = write!(buffer, "{}", if self.no_closing { ">".to_string() } else { "/>".to_string() });
      }
    } else {
      let _ = writeln!(buffer, ">");
      for (i, child) in self.children.iter().enumerate() {
        if i > 0 {
          let _ = writeln!(buffer);
        }
        child.write(indentation + indent, indent, buffer);
      }
      let _ = write!(buffer, "\n{}</{}>", get_indentation(self.no_indent, indentation), self.name);
    }
  }
}

fn get_indentation(no_indent: bool, indent: usize) -> String {
  if no_indent {
    "".to_string()
  } else {
    " ".to_string().repeat(indent)
  }
}
