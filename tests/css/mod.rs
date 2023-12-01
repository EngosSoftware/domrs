use domrs::CssDocument;

mod borders;
mod colors;
mod document;
mod element;
mod fonts;
mod group;
mod numbers;
mod properties;
mod rules;
mod ruleset;
mod selector;
mod units;
mod values;

/// Utility function for comparing CSS documents.
fn eq(expected: &str, css: CssDocument) {
  assert_eq!(expected, css.to_string());
}
