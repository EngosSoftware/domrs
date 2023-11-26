///
pub fn get_indentation(no_indent: bool, indent: usize) -> String {
  if no_indent {
    "".to_string()
  } else {
    " ".to_string().repeat(indent)
  }
}
