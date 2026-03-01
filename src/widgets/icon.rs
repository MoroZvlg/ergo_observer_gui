use crate::widgets::{Button};
use egui::{Color32, Vec2};
use rand::Rng;

macro_rules! icon_from_path {
  ($path:literal) => {
    Icon::new($path, include_bytes!($path))
  };
}

pub const SETTINGS_ICON: Icon = icon_from_path!("../../assets/icons/settings_icon.svg");
pub const EXIT_ICON: Icon = icon_from_path!("../../assets/icons/close_icon.svg");

pub struct Icon {
  path: &'static str,
  bytes: &'static [u8],
  fill: Option<Color32>,
  max_size: Option<Vec2>,
}

impl Icon {
  pub const fn new(path: &'static str, bytes: &'static [u8]) -> Self {
    Self { path, bytes, fill: None, max_size: None }
  }

  pub fn as_image_source(&self) -> egui::ImageSource<'static> {
    egui::ImageSource::Bytes { uri: self.path.into(), bytes: self.bytes.into() }
  }

  pub fn fill(mut self, color: Color32) -> Self {
    self.fill = Some(color);
    self
  }

  pub fn max_size(mut self, size: f32) -> Self {
    self.max_size = Some(Vec2::splat(size));
    self
  }

  pub fn as_image(&self) -> egui::Image<'static> {
    let mut image = egui::Image::new(self.as_image_source());
    if self.fill.is_some() {
      image = image.tint(self.fill.unwrap());
    }
    if self.max_size.is_some() {
      image = image.max_size(self.max_size.unwrap());
    }
    image
  }

  pub fn as_button(&self) -> Button<'static> {
    Button::new(self.as_image())
  }
}
