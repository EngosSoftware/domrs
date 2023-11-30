use domrs::CssUnit;

#[test]
fn display_should_work() {
  assert_eq!("auto", CssUnit::Auto.to_string());
  assert_eq!("", CssUnit::None.to_string());
  assert_eq!("%", CssUnit::Perc.to_string());
  assert_eq!("cm", CssUnit::Cm.to_string());
  assert_eq!("mm", CssUnit::Mm.to_string());
  assert_eq!("Q", CssUnit::Q.to_string());
  assert_eq!("in", CssUnit::In.to_string());
  assert_eq!("pc", CssUnit::Pc.to_string());
  assert_eq!("pt", CssUnit::Pt.to_string());
  assert_eq!("px", CssUnit::Px.to_string());
  assert_eq!("em", CssUnit::Em.to_string());
  assert_eq!("ex", CssUnit::Ex.to_string());
  assert_eq!("ch", CssUnit::Ch.to_string());
  assert_eq!("rem", CssUnit::Rem.to_string());
  assert_eq!("lh", CssUnit::Lh.to_string());
  assert_eq!("rlh", CssUnit::Rlh.to_string());
  assert_eq!("vw", CssUnit::Vw.to_string());
  assert_eq!("vh", CssUnit::Vh.to_string());
  assert_eq!("vmin", CssUnit::Vmin.to_string());
  assert_eq!("vmax", CssUnit::Vmax.to_string());
  assert_eq!("vb", CssUnit::Vb.to_string());
  assert_eq!("vi", CssUnit::Vi.to_string());
  assert_eq!("svw", CssUnit::Svw.to_string());
  assert_eq!("svh", CssUnit::Svh.to_string());
  assert_eq!("lvw", CssUnit::Lvw.to_string());
  assert_eq!("lvh", CssUnit::Lvh.to_string());
  assert_eq!("dvw", CssUnit::Dvw.to_string());
  assert_eq!("dvh", CssUnit::Dvh.to_string());
}
