use super::*;
use domrs::{
  CssColor, CssDocument, CssFontFamily, CssFontGenericFamily, CssNumber, CssProperty, CssRuleset, CssSelector, CssUnit, CssValue, HtmlBodyElement, HtmlElement, HtmlHeadElement,
  HtmlLinkElement, HtmlStyleElement,
};

#[test]
fn default_should_work() {
  eq(H001, doc().head(HtmlHeadElement::default()));
}

#[test]
fn charset_should_work() {
  eq(H002, doc().head(HtmlHeadElement::default().charset("UTF-8")));
}

#[test]
fn title_should_work() {
  eq(H003, doc().head(HtmlHeadElement::default().title("TITLE")));
}

#[test]
fn link_should_work() {
  eq(
    H004,
    doc().head(HtmlHeadElement::default().link(HtmlLinkElement::default().rel("stylesheet").href("https://domrs.com/main.css"))),
  );
}

#[test]
fn style_should_work() {
  eq(
    H005,
    doc()
      .head(
        HtmlHeadElement::default()
          .title("TITLE")
          .stylesheet("https://fonts.googleapis.com/css2?family=Pacifico&display=swap")
          .style(HtmlStyleElement::new(
            CssDocument::new().ruleset(
              CssRuleset::new(CssSelector::new().class("my-text"))
                .declaration(CssProperty::FontFamily, CssFontFamily::new(&["Pacifico".to_string()], CssFontGenericFamily::Serif))
                .declaration(CssProperty::FontSize, CssNumber::new(40.0, 0, CssUnit::Pt))
                .declaration(CssProperty::Color, CssValue::Color(CssColor::BlueViolet)),
            ),
          )),
      )
      .body(HtmlBodyElement::default().child(HtmlElement::span().class("my-text").content("Document builder and serializer"))),
  );
}

#[test]
fn style_with_custom_indent_should_work() {
  eq(
    H006,
    doc()
      .head(
        HtmlHeadElement::default()
          .title("TITLE")
          .link(HtmlLinkElement::default().stylesheet("https://fonts.googleapis.com/css2?family=Pacifico&display=swap"))
          .style(
            HtmlStyleElement::new(
              CssDocument::new().ruleset(
                CssRuleset::new(CssSelector::new().class("my-text"))
                  .declaration(CssProperty::FontFamily, CssFontFamily::new(&["Pacifico".to_string()], CssFontGenericFamily::Serif))
                  .declaration(CssProperty::FontSize, CssNumber::new(40.0, 0, CssUnit::Pt))
                  .declaration(CssProperty::Color, CssValue::Color(CssColor::BlueViolet)),
              ),
            )
            .indent(4),
          ),
      )
      .body(HtmlBodyElement::default().child(HtmlElement::new("span").attr("class", "my-text").content("Document builder and serializer"))),
  );
}
