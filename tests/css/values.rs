use domrs::{border, px, CssBorder, CssBorderStyle, CssColor, CssFontFamily, CssFontGenericFamily, CssFontStyle, CssNumber, CssUnit, CssValue};

#[test]
fn display_should_work() {
  let eq = |expected: &str, value: CssValue| {
    assert_eq!(expected, value.clone().to_string());
  };
  eq("avoid", CssValue::Avoid);
  eq("block", CssValue::Block);
  eq("1px solid black", CssValue::Border(border!(px!(1), CssBorderStyle::Solid, CssColor::Black)));
  eq("center", CssValue::Center);
  eq("blueviolet", CssValue::Color(CssColor::BlueViolet));
  eq("column", CssValue::Column);
  eq("flex", CssValue::Flex);
  eq("flex-start", CssValue::FlexStart);
  eq(
    r#""Helvetica", sans-serif"#,
    CssValue::FontFamily(CssFontFamily::new(&["Helvetica".to_string()], CssFontGenericFamily::SansSerif)),
  );
  eq("normal", CssValue::FontStyle(CssFontStyle::Normal));
  eq("grid", CssValue::Grid);
  eq("hidden", CssValue::Hidden);
  eq("inline-block", CssValue::InlineBlock);
  eq("7", CssValue::Integer(7));
  eq("left", CssValue::Left);
  eq("none", CssValue::None);
  eq("1px", CssValue::Num1(px!(1)));
  eq("1px 2px", CssValue::Num2(px!(1), px!(2)));
  eq("1px 2px 3px", CssValue::Num3(px!(1), px!(2), px!(3)));
  eq("1px 2px 3px 4px", CssValue::Num4(px!(1), px!(2), px!(3), px!(4)));
  eq("relative", CssValue::Relative);
  eq("right", CssValue::Right);
  eq("row", CssValue::Row);
  eq("start", CssValue::Start);
  eq("unset", CssValue::Unset);
  eq("0", CssValue::Zero);
}

#[test]
fn conversion_should_work() {
  let eq = |expected: &str, value: CssValue| {
    assert_eq!(expected, value.to_string());
  };
  eq("1px solid black", border!(px!(1), CssBorderStyle::Solid, CssColor::Black).into());
  eq("blueviolet", CssColor::BlueViolet.into());
  eq(
    r#""Helvetica", sans-serif"#,
    CssFontFamily::new(&["Helvetica".to_string()], CssFontGenericFamily::SansSerif).into(),
  );
  eq("normal", CssFontStyle::Normal.into());
  eq("7", 7_u8.into());
  eq("7", 7_i8.into());
  eq("7", 7_u16.into());
  eq("7", 7_i16.into());
  eq("7", 7_u32.into());
  eq("7", 7_i32.into());
  eq("7", 7_u64.into());
  eq("7", 7_i64.into());
  eq("-1", u64::MAX.into());
  eq("1px", px!(1).into());
  eq("1px 2px", (px!(1), px!(2)).into());
  eq("1px 2px 3px", (px!(1), px!(2), px!(3)).into());
  eq("1px 2px 3px 4px", (px!(1), px!(2), px!(3), px!(4)).into());
}
