use crate::common::{get_indentation, ToText};
use crate::{HtmlAttribute, DEFAULT_HTML_INDENT, DEFAULT_HTML_OFFSET};
use std::fmt;
use std::fmt::{Debug, Display, Write};

/// Structure representing HTML element.
#[derive(Debug, Clone)]
pub struct HtmlElement {
  /// Name of the element, will appear as a tag name in HTML document.
  name: String,
  /// Attributes of the element.
  attributes: Vec<HtmlAttribute>,
  /// Textual content of the element.
  content: Option<String>,
  /// Child elements.
  children: Vec<HtmlElement>,
  /// Flag indicating if closing tag will be serialized.
  hide_closing_tag: bool,
  /// Flag indicating if the element is serialized without indentation.
  no_indent: bool,
  /// Flag indicating if closing tag will be expanded when element is empty.
  always_expand: bool,
}

impl HtmlElement {
  /// Creates a new HTML element with specified tag name.
  pub fn new(name: &str) -> Self {
    Self {
      name: name.to_string(),
      attributes: vec![],
      content: None,
      children: vec![],
      hide_closing_tag: false,
      no_indent: false,
      always_expand: false,
    }
  }

  pub fn no_indent(mut self) -> Self {
    self.no_indent = true;
    self
  }

  pub fn hide_closing_tag(mut self) -> Self {
    self.hide_closing_tag = true;
    self
  }

  pub fn always_expand(mut self) -> Self {
    self.always_expand = true;
    self
  }

  /// Sets an attribute of the HTML element.
  pub fn attr<T: ToString>(mut self, name: &str, value: T) -> Self {
    self.set_attr(name, value);
    self
  }

  /// Sets an attribute of the HTML element.
  pub fn set_attr<T: ToString>(&mut self, name: &str, value: T) {
    self.attributes.push(HtmlAttribute {
      name: name.to_string(),
      value: value.to_string(),
    })
  }

  /// Sets a class attribute of the HTML element.
  pub fn class(mut self, class: &str) -> Self {
    self.set_class(class);
    self
  }

  /// Sets a class attribute of the HTML element.
  pub fn set_class(&mut self, class: &str) {
    self.attributes.push(HtmlAttribute {
      name: "class".to_string(),
      value: class.to_string(),
    })
  }

  /// Sets a style attribute of the HTML element.
  pub fn style(mut self, style: &str) -> Self {
    self.set_style(style);
    self
  }

  /// Sets a style attribute of the HTML element.
  pub fn set_style(&mut self, style: &str) {
    self.attributes.push(HtmlAttribute {
      name: "style".to_string(),
      value: style.to_string(),
    })
  }

  /// Adds a child element.
  pub fn child(mut self, child: impl Into<HtmlElement>) -> Self {
    self.add_child(child);
    self
  }

  /// Adds a child element.
  pub fn add_child(&mut self, child: impl Into<HtmlElement>) {
    self.children.push(child.into());
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

  pub fn content(mut self, content: &str) -> Self {
    self.set_content(content);
    self
  }

  /// Sets the content of the HTML element.
  pub fn set_content(&mut self, content: &str) {
    self.content = content.to_string().into();
  }

  /// Serializes the element to its textual representation.
  pub(crate) fn write(&self, mut offset: usize, indent: usize, buffer: &mut String) {
    if self.no_indent && offset >= indent {
      offset -= indent;
    }
    let _ = write!(buffer, "{}<{}", get_indentation(self.no_indent, offset), self.name);
    for attribute in &self.attributes {
      let _ = write!(buffer, r#" {}="{}""#, attribute.name, attribute.value);
    }
    if self.children.is_empty() {
      if let Some(content) = &self.content {
        let line_count = content.lines().count();
        if line_count > 1 {
          let _ = write!(buffer, ">");
          for line in content.lines() {
            let _ = write!(buffer, "\n{}{}", get_indentation(false, offset + indent), line);
          }
          let _ = write!(buffer, "\n{}</{}>", get_indentation(false, offset), self.name);
        } else {
          let _ = write!(buffer, ">{}</{}>", content, self.name);
        }
      } else {
        let _ = write!(
          buffer,
          "{}",
          if self.always_expand {
            format!("></{}>", self.name)
          } else if self.hide_closing_tag {
            ">".to_string()
          } else {
            "/>".to_string()
          }
        );
      }
    } else {
      let _ = writeln!(buffer, ">");
      for (i, child) in self.children.iter().enumerate() {
        if i > 0 {
          let _ = writeln!(buffer);
        }
        child.write(offset + indent, indent, buffer);
      }
      let _ = write!(buffer, "\n{}</{}>", get_indentation(self.no_indent, offset), self.name);
    }
  }
}

impl ToText for HtmlElement {
  /// Converts [HtmlElement] into text with specified offset and indent.
  fn to_text(&self, offset: usize, indent: usize) -> String {
    let mut buffer = String::new();
    self.write(offset, indent, &mut buffer);
    buffer
  }
}

impl Display for HtmlElement {
  /// Converts [HtmlElement] into text.
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.to_text(DEFAULT_HTML_OFFSET, DEFAULT_HTML_INDENT))
  }
}

/// Implementation of commonly used HTML elements.
impl HtmlElement {
  /// Creates `<h1>` HTML element. The [section heading] level 1 element.
  ///
  /// [section heading]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/Heading_Elements
  ///
  /// # Example
  ///
  /// ```
  /// # use domrs::HtmlElement;
  /// let h = HtmlElement::h1("Heading level 1");
  /// assert_eq!("<h1>Heading level 1</h1>", h.to_string());
  /// ```
  pub fn h1(content: &str) -> Self {
    Self::new("h1").content(content)
  }

  /// Creates `<h2>` HTML element. The [section heading] level 2 element.
  ///
  /// [section heading]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/Heading_Elements
  ///
  /// # Example
  ///
  /// ```
  /// # use domrs::HtmlElement;
  /// let h = HtmlElement::h2("Heading level 2");
  /// assert_eq!("<h2>Heading level 2</h2>", h.to_string());
  /// ```
  pub fn h2(content: &str) -> Self {
    Self::new("h2").content(content)
  }

  /// Creates `<h3>` HTML element. The [section heading] level 3 element.
  ///
  /// [section heading]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/Heading_Elements
  ///
  /// # Example
  ///
  /// ```
  /// # use domrs::HtmlElement;
  /// let h = HtmlElement::h3("Heading level 3");
  /// assert_eq!("<h3>Heading level 3</h3>", h.to_string());
  /// ```
  pub fn h3(content: &str) -> Self {
    Self::new("h3").content(content)
  }

  /// Creates `<h4>` HTML element. The [section heading] level 4 element.
  ///
  /// [section heading]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/Heading_Elements
  ///
  /// # Example
  ///
  /// ```
  /// # use domrs::HtmlElement;
  /// let h = HtmlElement::h4("Heading level 4");
  /// assert_eq!("<h4>Heading level 4</h4>", h.to_string());
  /// ```
  pub fn h4(content: &str) -> Self {
    Self::new("h4").content(content)
  }

  /// Creates `<h5>` HTML element. The [section heading] level 5 element.
  ///
  /// [section heading]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/Heading_Elements
  ///
  /// # Example
  ///
  /// ```
  /// # use domrs::HtmlElement;
  /// let h = HtmlElement::h5("Heading level 5");
  /// assert_eq!("<h5>Heading level 5</h5>", h.to_string());
  /// ```
  pub fn h5(content: &str) -> Self {
    Self::new("h5").content(content)
  }

  /// Creates `<h6>` HTML element. The [section heading] level 6 element.
  ///
  /// [section heading]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/Heading_Elements
  ///
  /// # Example
  ///
  /// ```
  /// # use domrs::HtmlElement;
  /// let h = HtmlElement::h6("Heading level 6");
  /// assert_eq!("<h6>Heading level 6</h6>", h.to_string());
  /// ```
  pub fn h6(content: &str) -> Self {
    Self::new("h6").content(content)
  }

  /// Creates `<br>` HTML element. The [line break] element.
  ///
  /// [line break]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/br
  ///
  /// # Example
  ///
  /// ```
  /// # use domrs::HtmlElement;
  /// let br = HtmlElement::br();
  /// assert_eq!("<br/>", br.to_string());
  /// ```
  pub fn br() -> Self {
    Self::new("br")
  }

  /// Creates `<div>` HTML element. The [content division] element.
  ///
  /// [content division]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/div
  ///
  /// # Example
  ///
  /// ```
  /// # use domrs::HtmlElement;
  /// let div = HtmlElement::div();
  /// assert_eq!("<div></div>", div.to_string());
  /// ```
  pub fn div() -> Self {
    Self::new("div").always_expand()
  }

  /// Creates `<span>` HTML element. The [content span] element.
  ///
  /// [content span]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/span
  ///
  /// # Example
  ///
  /// ```
  /// # use domrs::HtmlElement;
  /// let span = HtmlElement::span();
  /// assert_eq!("<span></span>", span.to_string());
  /// ```
  pub fn span() -> Self {
    Self::new("span").always_expand()
  }
}

/// Creates `<h1>` HTML element. The section heading level 1 element.
///
/// # Example
///
/// ```
/// # use domrs::h1;
/// # use domrs::HtmlElement;
/// let h = h1!("Heading level 1");
/// assert_eq!("<h1>Heading level 1</h1>", h.to_string());
/// ```
#[macro_export]
macro_rules! h1 {
  ($content:expr) => {
    HtmlElement::h1($content)
  };
}

/// Creates `<h2>` HTML element. The section heading level 2 element.
///
/// # Example
///
/// ```
/// # use domrs::h2;
/// # use domrs::HtmlElement;
/// let h = h2!("Heading level 2");
/// assert_eq!("<h2>Heading level 2</h2>", h.to_string());
/// ```
#[macro_export]
macro_rules! h2 {
  ($content:expr) => {
    HtmlElement::h2($content)
  };
}

/// Creates `<h3>` HTML element. The section heading level 3 element.
///
/// # Example
///
/// ```
/// # use domrs::h3;
/// # use domrs::HtmlElement;
/// let h = h3!("Heading level 3");
/// assert_eq!("<h3>Heading level 3</h3>", h.to_string());
/// ```
#[macro_export]
macro_rules! h3 {
  ($content:expr) => {
    HtmlElement::h3($content)
  };
}

/// Creates `<h4>` HTML element. The section heading level 4 element.
///
/// # Example
///
/// ```
/// # use domrs::h4;
/// # use domrs::HtmlElement;
/// let h = h4!("Heading level 4");
/// assert_eq!("<h4>Heading level 4</h4>", h.to_string());
/// ```
#[macro_export]
macro_rules! h4 {
  ($content:expr) => {
    HtmlElement::h4($content)
  };
}

/// Creates `<h5>` HTML element. The section heading level 5 element.
///
/// # Example
///
/// ```
/// # use domrs::h5;
/// # use domrs::HtmlElement;
/// let h = h5!("Heading level 5");
/// assert_eq!("<h5>Heading level 5</h5>", h.to_string());
/// ```
#[macro_export]
macro_rules! h5 {
  ($content:expr) => {
    HtmlElement::h5($content)
  };
}

/// Creates `<h6>` HTML element. The section heading level 6 element.
///
/// # Example
///
/// ```
/// # use domrs::h6;
/// # use domrs::HtmlElement;
/// let h = h6!("Heading level 6");
/// assert_eq!("<h6>Heading level 6</h6>", h.to_string());
/// ```
#[macro_export]
macro_rules! h6 {
  ($content:expr) => {
    HtmlElement::h6($content)
  };
}

#[macro_export]
macro_rules! div {
  ($class:expr, $content:expr) => {
    HtmlElement::div().class($class).content($content)
  };
  ($class:expr) => {
    HtmlElement::div().class($class)
  };
  () => {
    HtmlElement::div()
  };
}
