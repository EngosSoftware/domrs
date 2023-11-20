mod borders;
mod colors;
mod declarations;
mod document;
mod element;
mod group;
mod properties;
mod ruleset;
mod selector;
mod types;
mod units;
mod values;

pub use borders::{CssBorderStyle, CssBorderWidth};
pub use colors::CssColor;
pub use declarations::CssDeclaration;
pub use document::CssDocument;
pub use element::CssElement;
pub use group::{CssAtRule, CssGroup};
pub use properties::CssProperty;
pub use ruleset::CssRuleset;
pub use selector::{CssSelector, CssSelectorComponent, CssSelectorPart};
pub use types::CssType;
pub use units::CssUnit;
pub use values::{CssNumber, CssValue};

/// Default offset in CSS document.
pub const DEFAULT_CSS_OFFSET: usize = 0;

/// Default indentation in CSS document.
pub const DEFAULT_CSS_INDENT: usize = 4;
