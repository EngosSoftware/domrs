use domrs::SvgNumber;

fn eq(expected: &str, number: SvgNumber) {
  assert_eq!(expected, number.to_string());
}

#[test]
fn display_should_work() {
  eq("1", SvgNumber::new(1.0, 0));
  eq("1", SvgNumber::new(1.23456789, 0));
  eq("1.2", SvgNumber::new(1.23456789, 1));
  eq("1.2345679", SvgNumber::new(1.23456789, 7));
}

#[test]
fn conversions_should_work() {
  eq("1", 1_u8.into());
  eq("1", 1_i8.into());
  eq("1", 1_u16.into());
  eq("1", 1_i16.into());
  eq("1", 1_u32.into());
  eq("1", 1_i32.into());
  eq("1", 1_u64.into());
  eq("1", 1_i64.into());
  eq("1", 1_f32.into());
  eq("1", 1_f64.into());
}
