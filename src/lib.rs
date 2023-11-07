mod headings;
mod html_attribute;
mod html_body_element;
mod html_document;
mod html_element;
mod html_head_element;
mod html_link_element;

mod svg_element;

pub use headings::HeadingLevel;
pub use html_attribute::HtmlAttribute;
pub use html_body_element::HtmlBodyElement;
pub use html_document::HtmlDocument;
pub use html_element::HtmlElement;
pub use html_head_element::HtmlHeadElement;
pub use html_link_element::HtmlLinkElement;
pub use svg_element::SvgElement;

#[cfg(test)]
mod tests;
