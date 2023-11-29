pub trait ToText {
  /// Converts the implementor into text representation,
  /// applying provided `offset` and `intent` argument during formatting.
  fn to_text(&self, offset: usize, indent: usize) -> String;
}

///
pub fn get_indentation(no_indent: bool, indent: usize) -> String {
  if no_indent {
    "".to_string()
  } else {
    " ".to_string().repeat(indent)
  }
}
