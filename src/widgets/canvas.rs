pub mod transform;
pub mod actor;
pub mod actor_connection;
pub mod actor_group;
pub mod drawings;

pub use transform::ViewportTransform;
pub use actor::Actor;
pub use actor_connection::ActorConnection;
pub use actor_group::ActorGroup;
pub use drawings::CanvasDrawings;

use eframe::epaint::{Color32, CornerRadius, Stroke, StrokeKind};
use egui::{Id, Pos2, Rect, Sense, Vec2, FontId, Align2};

const ZOOM_SMOOTH: f32 = 1000.0;
const MIN_ZOOM: f32 = 0.1;
const MAX_ZOOM: f32 = 3.0;
const DEFAULT_WIDTH: f32 = 50_000.0;
const DEFAULT_HEIGHT: f32 = 50_000.0;

pub struct Canvas {
  id: Id,
  size: Rect,

  viewport_bg_color: Color32,
  viewport_stroke: Stroke,
  viewport_stroke_kind: StrokeKind,
  viewport_stroke_radius: CornerRadius,

  debug: bool,

  grid_step: u8,
  grid_color: Color32,
}

impl Default for Canvas {
  fn default() -> Self {
    Self {
      id: Id::new("canvas"),
      size: Rect::from_min_max(Pos2::new(-DEFAULT_WIDTH/2.0, -DEFAULT_HEIGHT/2.0), Pos2::new(DEFAULT_WIDTH/2.0, DEFAULT_HEIGHT/2.0)),

      viewport_bg_color: Color32::BLACK,
      viewport_stroke: Stroke::new(2.0, Color32::GRAY),
      viewport_stroke_kind: StrokeKind::Inside,
      viewport_stroke_radius: CornerRadius::ZERO,

      debug: true,
      grid_step: 10,
      grid_color: Color32::GRAY,
    }
  }
}

impl Canvas {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn bg_color(mut self, color: Color32) -> Self {
    self.viewport_bg_color = color;
    self
  }

  pub fn bg_stroke(mut self, stroke: Stroke) -> Self {
    self.viewport_stroke = stroke;
    self
  }

  pub fn grid_color(mut self, color: Color32) -> Self {
    self.grid_color = color;
    self
  }

  pub fn show(&self, ui: &mut egui::Ui, drawings: &mut CanvasDrawings) {
    let viewport_rect = ui.available_rect_before_wrap();
    let viewport_size = viewport_rect.size();
    let zoom_factor = self.get_zoom(ui);
    let drag_offset = self.get_offset(ui);

    // offset from app window corder to viewport corner (original pixels)
    let viewport_window_offset = viewport_rect.min.to_vec2();
    // offset from viewport corner to viewport center (original pixels)
    let viewport_center_offset = viewport_size / 2.0;
    // offset from viewport corner to viewport center (canvas pixels)
    let viewport_center_canvas_offset = viewport_center_offset / zoom_factor;

    let transform = ViewportTransform {
      origin: viewport_window_offset,
      center_offset: viewport_center_canvas_offset, // pass origin? consider zoom inside?
      drag_offset: drag_offset,                     // but this one already consider zoom
      zoom: zoom_factor,
    };

    let visible_canvas_rect = Rect::from_min_max(
      Pos2::ZERO - viewport_center_canvas_offset - drag_offset,
      Pos2::ZERO + viewport_center_canvas_offset - drag_offset,
    );

    // allocate area
    let (response, painter) = ui.allocate_painter(viewport_size, Sense::all());
    painter.rect_filled(
      response.rect,
      self.viewport_stroke_radius,
      self.viewport_bg_color,
    );
    painter.rect_stroke(
      response.rect,
      self.viewport_stroke_radius,
      self.viewport_stroke,
      self.viewport_stroke_kind,
    );

    if self.debug {
      // 0, 0 lines
      painter.text(
        viewport_rect.min + Vec2::splat(10.0),
        Align2::LEFT_TOP,
        format!(
          "Zoom - {:?}\nOffset - {:?}",
          zoom_factor,
          drag_offset
        ),
        FontId::default(),
        Color32::WHITE.gamma_multiply(0.25),
      );
    }

    if response.hovered() {
      let scroll_delta = ui.input(|i| i.smooth_scroll_delta.y);
      if scroll_delta != 0.0 {
        let zoom_factor = (zoom_factor + scroll_delta / ZOOM_SMOOTH).min(MAX_ZOOM).max(MIN_ZOOM);
        self.store_zoom(ui, zoom_factor);
      }


      // handle dragging
      let mut drag_offset = self.get_offset(ui);

      if response.dragged() {
        drag_offset += response.drag_delta() / zoom_factor;
        if drag_offset.y > self.size.max.y - viewport_center_canvas_offset.y {
          drag_offset.y = self.size.max.y - viewport_center_canvas_offset.y;
        }
        if drag_offset.y < self.size.min.y + viewport_center_canvas_offset.y {
          drag_offset.y = self.size.min.y + viewport_center_canvas_offset.y;
        }
        if drag_offset.x > self.size.max.x - viewport_center_canvas_offset.x {
          drag_offset.x = self.size.max.x - viewport_center_canvas_offset.x;
        }
        if drag_offset.x < self.size.min.x + viewport_center_canvas_offset.x {
          drag_offset.x = self.size.min.x + viewport_center_canvas_offset.x;
        }
        self.store_offset(ui, drag_offset);
      }

      if self.debug {
        let cursor_position = ui.input(|i| i.pointer.hover_pos());
        if cursor_position.is_some() {
          painter.text(
            cursor_position.unwrap() + Vec2::splat(10.0),
            Align2::LEFT_TOP,
            format!(
              "Screen - {:?}\nCanvas - {:?}",
              cursor_position.unwrap(),
              transform.to_canvas(cursor_position.unwrap()),
            ),
            FontId::default(),
            Color32::WHITE.gamma_multiply(0.25),
          );
        }
      }
    }

    // Draw grid
    painter.hline(
      viewport_rect.x_range(),
      transform.to_screen(Pos2::ZERO).y,
      Stroke::new(0.5, self.grid_color.gamma_multiply(0.3)),
    );

    painter.vline(
      transform.to_screen(Pos2::ZERO).x,
      viewport_rect.y_range(),
      Stroke::new(0.5, self.grid_color.gamma_multiply(0.3)),
    );

    let mut step = self.grid_step as f32;
    // if we zoom out - increase grid step
    if zoom_factor < 1.0 {
      step = step * (1.0 + (1.0 - zoom_factor) * (100.0 / step));
      step = (step / 10.0_f32).trunc() * 10.0;
    }

    let mut current_x = (visible_canvas_rect.min.x / step).trunc() * step;
    while current_x <= visible_canvas_rect.max.x {
      // every 10th line is full color
      let opacity = if current_x % (step * 10.0) == 0.0 { 0.2 } else { 0.1 };
      painter.vline(
        transform.to_screen(Pos2::new(current_x, 0.0)).x,
        viewport_rect.y_range(),
        Stroke::new(0.5, self.grid_color.gamma_multiply(opacity)),
      );
      current_x += step;
    }

    let mut current_y = (visible_canvas_rect.min.y / step).trunc() * step;
    while current_y <= visible_canvas_rect.max.y {
      // every 10th line stronger color
      let opacity = if current_y % (step * 10.0) == 0.0 { 0.2 } else { 0.1 };
      painter.hline(
        viewport_rect.x_range(),
        transform.to_screen(Pos2::new(0.0, current_y)).y,
        Stroke::new(0.5, self.grid_color.gamma_multiply(opacity)),
      );
      current_y += step;
    }

    drawings.draw(ui, &transform);

    let button_size = Vec2::splat(25.0);
    let button_pos = viewport_rect.max - Vec2::splat(5.0) - button_size;
    let button_rect = Rect::from_min_size(button_pos, button_size);
    let response = ui.allocate_rect(button_rect, Sense::click());
    if response.clicked() {
      self.store_zoom(ui, 1.0);
    }
    if response.hovered() {
      ui.painter().rect_filled(
        button_rect.shrink(2.0),
        CornerRadius::same(1),
        self.viewport_stroke.color
      );
    }
    ui.painter().rect_stroke(
      button_rect,
      CornerRadius::same(1),
      self.viewport_stroke,
      StrokeKind::Inside
    );
    ui.painter().text(
      button_pos + button_size / 2.0,
      Align2::CENTER_CENTER,
      "Z0",
      FontId::default(),
      Color32::WHITE,
    );
  }

  fn store_zoom(&self, ui: &mut egui::Ui, zoom: f32) {
    ui.memory_mut(|memory| {
      memory.data.insert_persisted(self.id.with("zoom"), zoom);
    })
  }

  fn get_zoom(&self, ui: &mut egui::Ui) -> f32 {
    ui.memory_mut(|mem| *mem.data.get_persisted_mut_or(self.id.with("zoom"), 1.0f32))
  }

  fn store_offset(&self, ui: &mut egui::Ui, offset: Vec2) {
    ui.memory_mut(|memory| {
      memory.data.insert_persisted(self.id.with("offset"), offset);
    })
  }

  fn get_offset(&self, ui: &mut egui::Ui) -> Vec2 {
    ui.memory_mut(|mem| *mem.data.get_persisted_mut_or(self.id.with("offset"), Vec2::ZERO))
  }
}
