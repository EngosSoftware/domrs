mod attribute;
mod body;
mod document;
mod element;
mod head;
mod headings;
mod link;

pub use attribute::HtmlAttribute;
pub use body::HtmlBodyElement;
pub use document::HtmlDocument;
pub use element::HtmlElement;
pub use head::HtmlHeadElement;
pub use headings::HtmlHeadingLevel;
pub use link::HtmlLinkElement;

/// Default HTML namespace.
pub const DEFAULT_HTML_NAMESPACE: &str = "http://www.w3.org/1999/xhtml";

/// Default DOCTYPE.
pub const DEFAULT_HTML_DOCTYPE: &str = "<!DOCTYPE html>";

/// Default language.
pub const DEFAULT_HTML_LANGUAGE: &str = "en";

/// Default offset in HTML document.
pub const DEFAULT_HTML_OFFSET: usize = 0;

/// Default indentation in HTML document.
pub const DEFAULT_HTML_INDENT: usize = 2;
