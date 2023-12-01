mod attributes;
mod document;
mod numbers;

pub use attributes::SvgAttribute;
pub use document::SvgDocument;
pub use numbers::SvgNumber;

/// Default text offset in SVG documents.
pub const DEFAULT_SVG_OFFSET: usize = 0;

/// Default text indentation in SVG documents.
pub const DEFAULT_SVG_INDENT: usize = 2;
