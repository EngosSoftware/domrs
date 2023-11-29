mod attributes;
mod document;

pub use attributes::SvgAttribute;
pub use document::SvgDocument;

/// Default text offset in SVG documents.
pub const DEFAULT_SVG_OFFSET: usize = 0;

/// Default text indentation in SVG documents.
pub const DEFAULT_SVG_INDENT: usize = 2;
