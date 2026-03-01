use std::collections::{BTreeMap};
use std::sync::{Arc};
use egui::{Context, FontFamily, FontId, TextStyle, FontData, FontDefinitions, RichText};

pub trait FontExt {
  fn heading_s(self) -> RichText;
  fn heading_xs(self) -> RichText;
  fn text_xs(self) -> RichText;
}

impl FontExt for RichText {
  fn heading_s(self) -> RichText  {
    self.text_style(heading_s())
  }
  fn heading_xs(self) -> RichText  {
    self.text_style(heading_xs())
  }
  fn text_xs(self) -> RichText  {
    self.text_style(text_xs())
  }
}

pub fn heading_s() -> TextStyle{
  TextStyle::Name("HeadingS".into())
}

pub fn heading_xs() -> TextStyle{
  TextStyle::Name("HeadingXS".into())
}

pub fn text_xs() -> TextStyle{
  TextStyle::Name("TextXS".into())
}

pub fn apply_fonts(ctx: &Context) {
  use FontFamily::{Monospace, Proportional};

  let mut fonts = FontDefinitions::default();

  fonts.font_data.insert(
    "JetBrainsMono".to_owned(),
    Arc::new(FontData::from_static(include_bytes!(
      "../../assets/fonts/JetBrainsMono.ttf"
    ))),
  );

  fonts
    .families
    .entry(Monospace)
    .or_default()
    .insert(0, "JetBrainsMono".to_owned());
  fonts
    .families
    .entry(Proportional)
    .or_default()
    .insert(0, "JetBrainsMono".to_owned());

  ctx.set_fonts(fonts);

  let text_styles: BTreeMap<TextStyle, FontId> = [
    (text_xs(), FontId::new(10.0, Monospace)),
    (TextStyle::Small, FontId::new(12.0, Monospace)),
    (TextStyle::Monospace, FontId::new(13.0, Monospace)),
    (TextStyle::Body, FontId::new(14.0, Monospace)),
    (TextStyle::Button, FontId::new(14.0, Monospace)),
    (heading_xs(), FontId::new(15.0, Monospace)),
    (heading_s(), FontId::new(16.0, Monospace)),
    (TextStyle::Heading, FontId::new(17.0, Monospace)),
  ]
    .into();
  ctx.all_styles_mut(move |style| style.text_styles = text_styles.clone());
}