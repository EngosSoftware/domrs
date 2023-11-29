mod borders;
mod colors;
mod declarations;
mod document;
mod element;
mod fonts;
mod group;
mod numbers;
mod properties;
mod ruleset;
mod selector;
mod units;
mod values;

pub use borders::{CssBorder, CssBorderStyle};
pub use colors::CssColor;
pub use declarations::CssDeclaration;
pub use document::CssDocument;
pub use element::CssElement;
pub use fonts::{CssFontFamily, CssFontGenericFamily, CssFontStyle};
pub use group::{CssAtRule, CssGroup};
pub use numbers::CssNumber;
pub use properties::CssProperty;
pub use ruleset::CssRuleset;
pub use selector::{CssSelector, CssSelectorComponent, CssSelectorPart};
pub use units::CssUnit;
pub use values::CssValue;

/// Default text offset in CSS documents.
pub const DEFAULT_CSS_OFFSET: usize = 0;

/// Default text indentation in CSS documents.
pub const DEFAULT_CSS_INDENT: usize = 2;
