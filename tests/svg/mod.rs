use domrs::SvgDocument;

mod attributes;
mod document;
mod numbers;

/// Utility function for comparing SVG documents.
fn eq(expected: &str, svg: SvgDocument) {
  assert_eq!(expected, svg.to_string());
}
