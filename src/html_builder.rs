use std::fmt;
use std::fmt::Write;

/// Common indentation value.
const INDENT: usize = 2;

/// Reference of the `HTML` standard.
const HREF_XMLNS: &str = "http://www.w3.org/1999/xhtml";

/// Download link for normal font.
const HREF_FONT_NORMAL: &str = "https://fonts.googleapis.com/css2?family=Barlow:ital,wght@0,300;0,400;0,500;0,600;1,300;1,400;1,500;1,600&display=swap";

/// Download link for condensed font.
const HREF_FONT_CONDENSED: &str = "https://fonts.googleapis.com/css2?family=Barlow+Condensed:ital,wght@0,300;0,400;0,500;0,600;1,300;1,400;1,500;1,600&display=swap";

/// Download link for monospaced font.
const HREF_FONT_MONO: &str = "https://fonts.googleapis.com/css2?family=JetBrains+Mono:ital,wght@0,300;0,400;0,500;0,600;1,300;1,400;1,500;1,600&display=swap";

/// Definition of used `HTML` heading levels.
pub enum HeadingLevel {
  H1,
  H2,
  H3,
  H4,
  H5,
  H6,
}

/// Structure representing whole `HTML` document.
#[derive(Debug, Clone)]
pub struct HtmlDocument {
  root: HtmlElement,
}

impl HtmlDocument {
  ///
  pub fn new(title: &str, lang: &str, styles: &[&str], body: HtmlElement) -> Self {
    let mut root = HtmlElement::new("html");
    root.set_attr("lang", lang);
    root.set_attr("xmlns", HREF_XMLNS);
    // prepare HTML header
    let mut head = HtmlElement::new("head");
    // <meta>
    let mut meta = HtmlElement::new_void("meta");
    meta.set_attr("charset", "UTF-8");
    head.add_child(meta);
    // <title>
    let mut title_element = HtmlElement::new("title");
    title_element.set_content(title);
    head.add_child(title_element);
    // add link to normal font
    let mut link = HtmlElement::new_void("link");
    link.set_attr("rel", "stylesheet");
    link.set_attr("href", HREF_FONT_NORMAL);
    head.add_child(link);
    // add link to condensed font
    let mut link = HtmlElement::new_void("link");
    link.set_attr("rel", "stylesheet");
    link.set_attr("href", HREF_FONT_CONDENSED);
    head.add_child(link);
    // add link to monospaced font
    let mut link = HtmlElement::new_void("link");
    link.set_attr("rel", "stylesheet");
    link.set_attr("href", HREF_FONT_MONO);
    head.add_child(link);
    // <style>
    let mut style = HtmlElement::new("style");
    style.set_content(&styles.join("\n"));
    head.add_child(style);
    // finalize header
    root.add_child(head);
    // add HTML document body
    root.add_child(body);
    Self { root }
  }
}

impl fmt::Display for HtmlDocument {
  /// Converts `HTML` document into text.
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut buffer = String::new();
    let _ = writeln!(&mut buffer, "<!DOCTYPE html>");
    self.root.write(0, &mut buffer);
    write!(f, "{}", buffer)
  }
}

#[derive(Debug, Clone)]
struct HtmlAttribute {
  name: String,
  value: String,
}

#[derive(Debug, Clone)]
pub struct HtmlElement {
  name: String,
  attributes: Vec<HtmlAttribute>,
  content: Option<String>,
  children: Vec<HtmlElement>,
  void: bool,
}

impl fmt::Display for HtmlElement {
  /// Converts `HTML` element into text.
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut buffer = String::new();
    self.write(0, &mut buffer);
    write!(f, "{}", buffer)
  }
}

impl HtmlElement {
  /// Creates a new `HTML` element with specified name.
  pub fn new(name: &str) -> Self {
    Self {
      name: name.to_string(),
      attributes: vec![],
      content: None,
      children: vec![],
      void: false,
    }
  }

  /// Creates a new `HTML` void element with specified name.
  pub fn new_void(name: &str) -> Self {
    Self {
      name: name.to_string(),
      attributes: vec![],
      content: None,
      children: vec![],
      void: true,
    }
  }

  /// Creates a new `<div>` element.
  pub fn new_div(class: Option<&str>) -> Self {
    let mut element = Self::new("div");
    if let Some(class_name) = class {
      element.set_class(class_name);
    }
    element
  }

  /// Sets an attribute of the `HTML` element.
  pub fn set_attr<T: ToString>(&mut self, name: &str, value: T) {
    self.attributes.push(HtmlAttribute {
      name: name.to_string(),
      value: value.to_string(),
    })
  }

  /// Sets a `class` attribute of the `HTML` element.
  pub fn set_class(&mut self, class: &str) {
    self.attributes.push(HtmlAttribute {
      name: "class".to_string(),
      value: class.to_string(),
    })
  }

  /// Sets a `style` attribute of the `HTML` element.
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

  /// Sets the content of the `HTML` element.
  pub fn set_content(&mut self, content: &str) {
    self.content = Some(content.to_string());
  }

  /// Serializes the element to its textual representation.
  pub fn write(&self, indent: usize, buffer: &mut String) {
    let _ = write!(buffer, "{}<{}", indentation(indent), self.name);
    for attribute in &self.attributes {
      let _ = write!(buffer, r#" {}="{}""#, attribute.name, attribute.value);
    }
    if self.children.is_empty() {
      if let Some(content) = &self.content {
        let line_count = content.lines().count();
        if line_count > 1 {
          let _ = write!(buffer, ">");
          for line in content.lines() {
            let _ = write!(buffer, "\n{}{}", indentation(indent + INDENT), line);
          }
          let _ = write!(buffer, "\n{}</{}>", indentation(indent), self.name);
        } else {
          let _ = write!(buffer, ">{}</{}>", content, self.name);
        }
      } else {
        let _ = write!(buffer, "{}", if self.void { ">".to_string() } else { format!("></{}>", self.name) });
      }
    } else {
      let _ = writeln!(buffer, ">");
      for (i, child) in self.children.iter().enumerate() {
        if i > 0 {
          let _ = writeln!(buffer);
        }
        child.write(indent + INDENT, buffer);
      }
      let _ = write!(buffer, "\n{}</{}>", indentation(indent), self.name);
    }
  }
}

fn indentation(indent: usize) -> String {
  " ".to_string().repeat(indent)
}
