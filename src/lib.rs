mod html;
mod svg;

pub use html::headings::HeadingLevel;
pub use html::html_attribute::HtmlAttribute;
pub use html::html_body_element::HtmlBodyElement;
pub use html::html_document::{HtmlDocument, DEFAULT_HTML_DOCTYPE, DEFAULT_HTML_INDENT, DEFAULT_HTML_LANGUAGE, DEFAULT_HTML_NAMESPACE};
pub use html::html_element::HtmlElement;
pub use html::html_head_element::HtmlHeadElement;
pub use html::html_link_element::HtmlLinkElement;
pub use svg::svg_document::SvgDocument;
