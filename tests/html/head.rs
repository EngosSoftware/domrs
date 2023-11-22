use super::*;
use domrs::{CssColor, CssDocument, CssProperty, CssRuleset, CssSelector, CssUnit, CssValue, HtmlBodyElement, HtmlElement, HtmlHeadElement, HtmlLinkElement, HtmlStyleElement};

#[test]
fn default_should_work() {
  eq(H001, doc().with_head(HtmlHeadElement::default()));
}

#[test]
fn charset_should_work() {
  eq(H002, doc().with_head(HtmlHeadElement::default().with_charset("UTF-8")));
}

#[test]
fn title_should_work() {
  eq(H003, doc().with_head(HtmlHeadElement::default().with_title("TITLE")));
}

#[test]
fn link_should_work() {
  eq(
    H004,
    doc().with_head(HtmlHeadElement::default().with_link(HtmlLinkElement::default().with_rel("stylesheet").with_href("https://domrs.com/main.css"))),
  );
}

#[test]
fn style_should_work() {
  eq(
    H005,
    doc()
      .with_head(
        HtmlHeadElement::default()
          .with_title("TITLE")
          .with_link(HtmlLinkElement::default().with_stylesheet("https://fonts.googleapis.com/css2?family=Pacifico&display=swap"))
          .with_style(HtmlStyleElement::new(
            CssDocument::new().ruleset(
              CssRuleset::new(CssSelector::new().class("my-text"))
                .declaration(CssProperty::FontFamily, CssValue::Text("Pacifico, serif".to_string()))
                .declaration(CssProperty::FontSize, CssValue::Number((40.0, 0, CssUnit::Pt)))
                .declaration(CssProperty::Color, CssValue::Color(CssColor::BlueViolet)),
            ),
          )),
      )
      .with_body(HtmlBodyElement::default().child(HtmlElement::new("span").attr("class", "my-text").content("DOM builder and serializer".to_string()))),
  );
}

#[test]
fn style_with_custom_indent_should_work() {
  eq(
    H006,
    doc()
      .with_head(
        HtmlHeadElement::default()
          .with_title("TITLE")
          .with_link(HtmlLinkElement::default().with_stylesheet("https://fonts.googleapis.com/css2?family=Pacifico&display=swap"))
          .with_style(HtmlStyleElement::new_indent(
            CssDocument::new().ruleset(
              CssRuleset::new(CssSelector::new().class("my-text"))
                .declaration(CssProperty::FontFamily, CssValue::Text("Pacifico, serif".to_string()))
                .declaration(CssProperty::FontSize, CssValue::Number((40.0, 0, CssUnit::Pt)))
                .declaration(CssProperty::Color, CssValue::Color(CssColor::BlueViolet)),
            ),
            4,
          )),
      )
      .with_body(HtmlBodyElement::default().child(HtmlElement::new("span").attr("class", "my-text").content("DOM builder and serializer".to_string()))),
  );
}
