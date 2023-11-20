use crate::CssNumber;

///
pub fn get_indentation(no_indent: bool, indent: usize) -> String {
  if no_indent {
    "".to_string()
  } else {
    " ".to_string().repeat(indent)
  }
}

///
pub fn number_to_string((value, precision, unit): CssNumber) -> String {
  format!("{0:.1$}{2}", value, precision, unit)
}
