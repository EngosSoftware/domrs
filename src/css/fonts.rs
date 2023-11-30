use std::fmt;
use std::fmt::Display;

/// An enumeration representing a CSS font style.
#[derive(Debug, Clone)]
pub enum CssFontStyle {
  /// Normal style.
  Normal,
  /// Italic style.
  Italic,
  /// Oblique style.
  Oblique,
}

impl Display for CssFontStyle {
  /// Implements [Display] for [CssFontStyle].
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "{}",
      match self {
        CssFontStyle::Normal => "normal",
        CssFontStyle::Italic => "italic",
        CssFontStyle::Oblique => "oblique",
      }
    )
  }
}

#[derive(Debug, Clone)]
pub enum CssFontGenericFamily {
  Serif,
  SansSerif,
  Monospace,
  Cursive,
  Fantasy,
  SystemUi,
  UiSerif,
  UiSansSerif,
  UiMonospace,
  UiRounded,
  Emoji,
  Math,
  FangSong,
}

impl Display for CssFontGenericFamily {
  ///
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "{}",
      match self {
        CssFontGenericFamily::Serif => "serif",
        CssFontGenericFamily::SansSerif => "sans-serif",
        CssFontGenericFamily::Monospace => "monospace",
        CssFontGenericFamily::Cursive => "cursive",
        CssFontGenericFamily::Fantasy => "fantasy",
        CssFontGenericFamily::SystemUi => "system-ui",
        CssFontGenericFamily::UiSerif => "ui-serif",
        CssFontGenericFamily::UiSansSerif => "ui-sans-serif",
        CssFontGenericFamily::UiMonospace => "ui-monospace",
        CssFontGenericFamily::UiRounded => "ui-rounded",
        CssFontGenericFamily::Emoji => "emoji",
        CssFontGenericFamily::Math => "math",
        CssFontGenericFamily::FangSong => "fangsong",
      }
    )
  }
}

#[derive(Debug, Clone)]
pub struct CssFontFamily {
  names: Vec<String>,
  generic: CssFontGenericFamily,
}

impl CssFontFamily {
  pub fn new(fonts: &[String], generic: CssFontGenericFamily) -> Self {
    Self { names: fonts.to_vec(), generic }
  }
}

impl Display for CssFontFamily {
  ///
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut font_names = self.names.iter().map(|name| format!(r#""{}""#, name)).collect::<Vec<String>>();
    font_names.push(format!("{}", self.generic));
    write!(f, "{}", font_names.join(", "))
  }
}
