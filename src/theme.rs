pub mod colors;
pub mod fonts;

pub use colors::{ColorExt, Colors};
pub use fonts::{apply_fonts, FontExt};

use eframe::egui::{Id, Context};
use eframe::epaint::Color32;

#[derive(Clone, Debug)]
pub struct Theme {
  pub name: String,
  pub colors: Colors,
}

impl Theme {
  pub fn dark() -> Self {
    Self { name: "Dark".to_string(), colors: Colors::default() }
  }

  pub fn light() -> Self {
    Self { name: "Light".to_string(), colors: Colors::light() }
  }

  pub fn apply(&self, ctx: &Context) {
    ctx.data_mut(|d| d.insert_temp(Id::new("custom_theme"), self.clone()));
  }

  pub fn get_theme(ctx: &Context) -> Theme {
    ctx
      .data(|d| d.get_temp::<Theme>(Id::new("custom_theme")).unwrap_or_else(|| Theme::dark()))
  }
}

impl Default for Theme {
  fn default() -> Self {
    Self::dark()
  }
}
