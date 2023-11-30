mod attributes;
mod body;
mod document;
mod element;
mod head;
mod link;
mod style;

pub use attributes::HtmlAttribute;
pub use body::HtmlBodyElement;
pub use document::HtmlDocument;
pub use element::HtmlElement;
pub use head::HtmlHeadElement;
pub use link::HtmlLinkElement;
pub use style::HtmlStyleElement;

/// Default namespace in HTML documents.
pub const DEFAULT_HTML_NAMESPACE: &str = "http://www.w3.org/1999/xhtml";

/// Default `DOCTYPE` in HTML documents.
pub const DEFAULT_HTML_DOCTYPE: &str = "<!DOCTYPE html>";

/// Default language for HTML documents.
pub const DEFAULT_HTML_LANGUAGE: &str = "en";

/// Default text offset in HTML documents.
pub const DEFAULT_HTML_OFFSET: usize = 0;

/// Default text indentation in HTML documents.
pub const DEFAULT_HTML_INDENT: usize = 2;
