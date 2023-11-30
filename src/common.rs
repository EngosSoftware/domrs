//! # Common definitions used across this library

/// A trait for converting value to a textual representation with provided offset and indentation.
pub trait ToText {
  /// Converts the implementing type to a text representation.
  ///
  /// The `offset` argument is the initial offset to be applied before the content.
  /// The `indent` argument is the indentation to be used for formatting structured content.
  /// Returns a [String] containing the textual representation of the implementing type.
  ///
  /// # Example
  ///
  /// ```
  /// # struct MyStruct;
  /// use domrs::ToText;
  ///
  /// impl ToText for MyStruct {
  ///     fn to_text(&self, offset: usize, indent: usize) -> String {
  ///         // Implementation logic comes here
  ///         String::new()
  ///     }
  /// }
  /// ```
  fn to_text(&self, offset: usize, indent: usize) -> String;
}

pub fn get_indentation(no_indent: bool, indent: usize) -> String {
  if no_indent {
    "".to_string()
  } else {
    " ".to_string().repeat(indent)
  }
}
