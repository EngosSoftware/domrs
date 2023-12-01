//! # Overview
//!
//! **domrs** is a powerful library designed to streamline the creation
//! and serialization of HTML, CSS and SVG documents.
//!
//! **domrs** empowers developers with a concise and intuitive interface to effortlessly
//! construct structured and visually appealing documents.
//!
//! Key features:
//!
//! - **HTML, CSS and SVG support**: seamlessly build HTML web pages, stylized CSS documents
//!   and dynamic SVG graphics using Rust's strong typing and safety.
//!
//! - **Builder pattern**: enjoy the convenience of a builder pattern for creating complex
//!   document structures, ensuring code readability and maintainability.
//!
//! - **Serialization**: effortlessly serialize created documents into standard-compliant
//!   HTML, CSS or SVG files, facilitating easy integration into web applications,
//!   storage or reporting tools.
//!
//! With its ergonomic design and robust functionality, **domrs** offers a hassle-free
//! solution for developers seeking a reliable tool to craft web-based documents within Rust projects.
//! Whether you're building a web app, generating dynamic graphics, or managing stylesheets,
//! **domrs** provides the tools you need.
//!
//! Get started with **domrs** today and unlock the potential for efficient
//! document creation and serialization in Rust!
//!
//! # Getting started
//!
//! ## Create HTML document
//!
//! ```
//! use domrs::HtmlDocument;
//! let html = HtmlDocument::new();
//! assert_eq!("<html/>\n", html.to_string());
//! ```
//!
//! ## Create CSS stylesheet
//!
//! ```
//! use domrs::CssDocument;
//! let css = CssDocument::new();
//! assert_eq!("", css.to_string());
//! ```
//!
//! ## Create SVG graphics
//!
//! ```
//! use domrs::SvgDocument;
//! let svg = SvgDocument::new();
//! assert_eq!(r#"<svg/>"#, svg.to_string());
//! ```

// #![deny(missing_docs)]
// #![deny(rustdoc::broken_intra_doc_links)]

mod common;
mod css;
mod html;
mod svg;

pub use common::ToText;
pub use css::{
  CssAtRule, CssBorder, CssBorderStyle, CssColor, CssDeclaration, CssDocument, CssElement, CssFontFamily, CssFontGenericFamily, CssFontStyle, CssGroup, CssNumber, CssProperty,
  CssRuleset, CssSelector, CssSelectorPart, CssUnit, CssValue, DEFAULT_CSS_INDENT, DEFAULT_CSS_OFFSET,
};
pub use html::{
  HtmlAttribute, HtmlBodyElement, HtmlDocument, HtmlElement, HtmlHeadElement, HtmlLinkElement, HtmlStyleElement, DEFAULT_HTML_DOCTYPE, DEFAULT_HTML_INDENT, DEFAULT_HTML_LANGUAGE,
  DEFAULT_HTML_NAMESPACE, DEFAULT_HTML_OFFSET,
};
pub use svg::{SvgAttribute, SvgDocument, SvgNumber, DEFAULT_SVG_INDENT, DEFAULT_SVG_OFFSET};
