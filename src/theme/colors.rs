use egui::Color32;

pub trait ColorExt {
  fn lighten(&self, amount: f32) -> Color32;
  fn darken(&self, amount: f32) -> Color32;
}

impl ColorExt for Color32 {
  fn lighten(&self, amount: f32) -> egui::Color32 {
    let rgba = egui::Rgba::from(*self);
    let r = rgba.r() + (1.0 - rgba.r()) * amount.clamp(0.0, 1.0);
    let g = rgba.g() + (1.0 - rgba.g()) * amount.clamp(0.0, 1.0);
    let b = rgba.b() + (1.0 - rgba.b()) * amount.clamp(0.0, 1.0);
    egui::Rgba::from_rgb(r, g, b).into()
  }

  fn darken(&self, amount: f32) -> egui::Color32 {
    let rgba = egui::Rgba::from(*self);
    let amount = amount.clamp(0.0, 1.0);
    egui::Rgba::from_rgb(
      rgba.r() * (1.0 - amount),
      rgba.g() * (1.0 - amount),
      rgba.b() * (1.0 - amount),
    )
    .into()
  }
}

#[derive(Clone, Debug)]
pub struct Colors {
  pub primary: Color32,
  pub on_primary: Color32,
  pub primary_container: Color32,
  pub on_primary_container: Color32,

  pub secondary: Color32,
  pub on_secondary: Color32,
  pub secondary_container: Color32,
  pub on_secondary_container: Color32,

  pub surface_dim: Color32,
  pub surface: Color32,
  pub surface_bright: Color32,

  // pub surface_container_lowest: Color32,
  pub surface_container_low: Color32,
  pub surface_container: Color32,
  pub surface_container_high: Color32,
  // pub surface_container_highest: Color32,

  pub on_surface: Color32,
  pub on_surface_variant: Color32,

  pub outline: Color32,
  pub outline_variant: Color32,

  // pub surface_inverse: Color32,
  // pub on_surface_inverse: Color32,
  // pub primary_inverse: Color32,
  // pub scrim: Color32,
  // pub shadow: Color32,
  pub success: Color32,
  pub on_success: Color32,
  pub success_container: Color32,
  pub on_success_container: Color32,

  pub error: Color32,
  pub on_error: Color32,
  pub error_container: Color32,
  pub on_error_container: Color32,
}

impl Colors {
  pub fn dark() -> Self {
    Self {
      primary: Color32::from_hex("#aac7ff").unwrap(),
      on_primary: Color32::from_hex("#0a305f").unwrap(),
      primary_container: Color32::from_hex("#284777").unwrap(),
      on_primary_container: Color32::from_hex("#d6e3ff").unwrap(),

      secondary: Color32::from_hex("#bec6dc").unwrap(),
      on_secondary: Color32::from_hex("#283141").unwrap(),
      secondary_container: Color32::from_hex("#3e4759").unwrap(),
      on_secondary_container: Color32::from_hex("#dae2f9").unwrap(),

      surface_dim: Color32::from_hex("#111318").unwrap(),
      surface: Color32::from_hex("#111318").unwrap(),
      surface_bright: Color32::from_hex("#37393e").unwrap(),

      // surface_container_lowest: Color32::from_hex("#0c0e13").unwrap(),
      surface_container_low: Color32::from_hex("#191c20").unwrap(),
      surface_container: Color32::from_hex("#1d2024").unwrap(),
      surface_container_high: Color32::from_hex("#282a2f").unwrap(),
      // surface_container_highest: Color32::from_hex("#33353a").unwrap(),

      on_surface: Color32::from_hex("#e2e2e9").unwrap(),
      on_surface_variant: Color32::from_hex("#c4c6d0").unwrap(),

      outline: Color32::from_hex("#8e9099").unwrap(),
      outline_variant: Color32::from_hex("#44474e").unwrap(),

      // surface_inverse: Color32::from_hex("#e2e2e9").unwrap(),
      // on_surface_inverse: Color32::from_hex("#2e3036").unwrap(),
      // primary_inverse: Color32::from_hex("#415f91").unwrap(),
      // scrim: Color32::from_hex("#000000").unwrap(),
      // shadow: Color32::from_hex("#000000").unwrap(),
      success: Color32::from_hex("#006d3a").unwrap(),
      on_success: Color32::from_hex("#ffffff").unwrap(),
      success_container: Color32::from_hex("#89f6a7").unwrap(),
      on_success_container: Color32::from_hex("#002110").unwrap(),

      error: Color32::from_hex("#ffb4ab").unwrap(),
      on_error: Color32::from_hex("#690005").unwrap(),
      error_container: Color32::from_hex("#93000a").unwrap(),
      on_error_container: Color32::from_hex("#ffdad6").unwrap(),
    }
  }

  pub fn light() -> Self {
    Self {
      primary: Color32::from_hex("#415f91").unwrap(),
      on_primary: Color32::from_hex("#ffffff").unwrap(),
      primary_container: Color32::from_hex("#d6e3ff").unwrap(),
      on_primary_container: Color32::from_hex("#284777").unwrap(),

      secondary: Color32::from_hex("#565f71").unwrap(),
      on_secondary: Color32::from_hex("#ffffff").unwrap(),
      secondary_container: Color32::from_hex("#dae2f9").unwrap(),
      on_secondary_container: Color32::from_hex("#3e4759").unwrap(),

      surface_dim: Color32::from_hex("#d9d9e0").unwrap(),
      surface: Color32::from_hex("#f9f9ff").unwrap(),
      surface_bright: Color32::from_hex("#f9f9ff").unwrap(),

      // surface_container_lowest: Color32::from_hex("#ffffff").unwrap(),
      surface_container_low: Color32::from_hex("#f3f3fa").unwrap(),
      surface_container: Color32::from_hex("#ededf4").unwrap(),
      surface_container_high: Color32::from_hex("#e7e8ee").unwrap(),
      // surface_container_highest: Color32::from_hex("#e2e2e9").unwrap(),

      on_surface: Color32::from_hex("#191c20").unwrap(),
      on_surface_variant: Color32::from_hex("#44474e").unwrap(),

      outline: Color32::from_hex("#74777f").unwrap(),
      outline_variant: Color32::from_hex("#c4c6d0").unwrap(),

      // surface_inverse: Color32::from_hex("#2e3036").unwrap(),
      // on_surface_inverse: Color32::from_hex("#f0f0f7").unwrap(),
      // primary_inverse: Color32::from_hex("#aac7ff").unwrap(),
      // scrim: Color32::from_hex("#000000").unwrap(),
      // shadow: Color32::from_hex("#000000").unwrap(),
      success: Color32::from_hex("#6dd98c").unwrap(),
      on_success: Color32::from_hex("#003919").unwrap(),
      success_container: Color32::from_hex("#005227").unwrap(),
      on_success_container: Color32::from_hex("#89f6a7").unwrap(),

      error: Color32::from_hex("#ba1a1a").unwrap(),
      on_error: Color32::from_hex("#ffffff").unwrap(),
      error_container: Color32::from_hex("#ffdad6").unwrap(),
      on_error_container: Color32::from_hex("#93000a").unwrap(),
    }
  }
}

impl Default for Colors {
  fn default() -> Self {
    Self::dark()
  }
}
