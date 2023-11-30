use domrs::{CssFontGenericFamily, CssFontStyle};

mod css_font_style_tests {
  use super::*;

  #[test]
  fn display_should_work() {
    assert_eq!("normal", CssFontStyle::Normal.clone().to_string());
    assert_eq!("italic", CssFontStyle::Italic.clone().to_string());
    assert_eq!("oblique", CssFontStyle::Oblique.clone().to_string());
  }
}

mod css_font_generic_family_tests {
  use super::*;

  #[test]
  fn display_should_work() {
    assert_eq!("serif", CssFontGenericFamily::Serif.clone().to_string());
    assert_eq!("sans-serif", CssFontGenericFamily::SansSerif.clone().to_string());
    assert_eq!("monospace", CssFontGenericFamily::Monospace.clone().to_string());
    assert_eq!("cursive", CssFontGenericFamily::Cursive.clone().to_string());
    assert_eq!("fantasy", CssFontGenericFamily::Fantasy.clone().to_string());
    assert_eq!("system-ui", CssFontGenericFamily::SystemUi.clone().to_string());
    assert_eq!("ui-serif", CssFontGenericFamily::UiSerif.clone().to_string());
    assert_eq!("ui-sans-serif", CssFontGenericFamily::UiSansSerif.clone().to_string());
    assert_eq!("ui-monospace", CssFontGenericFamily::UiMonospace.clone().to_string());
    assert_eq!("ui-rounded", CssFontGenericFamily::UiRounded.clone().to_string());
    assert_eq!("emoji", CssFontGenericFamily::Emoji.clone().to_string());
    assert_eq!("math", CssFontGenericFamily::Math.clone().to_string());
    assert_eq!("fangsong", CssFontGenericFamily::FangSong.clone().to_string());
  }
}
