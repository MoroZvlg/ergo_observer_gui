use egui::{Pos2, Rect, Vec2};

pub struct ViewportTransform {
  pub origin: Vec2,        // viewport_window_offset
  pub center_offset: Vec2, // viewport_center_offset
  pub drag_offset: Vec2,   // camera pan
  pub zoom: f32,
}

pub trait ToScreen {
  fn to_screen(self, t: &ViewportTransform) -> Self;
}

pub trait ToCanvas {
  fn to_canvas(self, t: &ViewportTransform) -> Self;
}

impl ViewportTransform {
  pub fn to_screen<T: ToScreen>(&self, value: T) -> T {
    value.to_screen(self)
  }

  pub fn to_canvas<T: ToCanvas>(&self, value: T) -> T {
    value.to_canvas(self)
  }
}

impl ToScreen for Pos2 {
  fn to_screen(self, t: &ViewportTransform) -> Self {
    (t.origin + ((self + t.center_offset + t.drag_offset) * t.zoom).to_vec2()).to_pos2()
  }
}

impl ToCanvas for Pos2 {
  fn to_canvas(self, t: &ViewportTransform) -> Self {
    (self - t.origin) / t.zoom - t.center_offset - t.drag_offset
  }
}

impl ToScreen for Rect {
  fn to_screen(self, t: &ViewportTransform) -> Self {
    Rect::from_min_max(self.min.to_screen(t), self.max.to_screen(t))
  }
}

impl ToCanvas for Rect {
  fn to_canvas(self, t: &ViewportTransform) -> Self {
    Rect::from_min_max(self.min.to_canvas(t), self.max.to_canvas(t))
  }
}

impl ToScreen for Vec2 {
  fn to_screen(self, t: &ViewportTransform) -> Self {
    self * t.zoom
  }
}

impl ToCanvas for Vec2 {
  fn to_canvas(self, t: &ViewportTransform) -> Self {
    self / t.zoom
  }
}

impl ToScreen for f32 {
  fn to_screen(self, t: &ViewportTransform) -> Self {
    self * t.zoom
  }
}

impl ToCanvas for f32 {
  fn to_canvas(self, t: &ViewportTransform) -> Self {
    self / t.zoom
  }
}
