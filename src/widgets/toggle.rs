use eframe::emath::{Pos2, Rect};
use eframe::epaint::{Stroke};
use egui::{Ui};
use crate::theme::Theme;

pub struct Toggle<'a> {
  value: &'a mut bool
}

impl<'a> Toggle<'a> {
  pub fn new(value: &'a mut bool) -> Toggle<'a> {
    Toggle { value }
  }

  pub fn show(&mut self, ui: &mut Ui) -> egui::Response {
    let desired_size = ui.spacing().interact_size;
    let (mut rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click());
    if response.clicked() {
      *self.value = !*self.value;
      response.mark_changed();
    }

    if response.hovered() {
      rect.min.x -= 1.0;
      rect.min.y -= 1.0;
      rect.max.x += 1.0;
      rect.max.y += 1.0;
    }

    response.widget_info(|| {
      egui::WidgetInfo::selected(egui::WidgetType::Checkbox, ui.is_enabled(), *self.value, "")
    });

    let theme = Theme::get_theme(&ui.ctx());
    if ui.is_rect_visible(rect) {
      let animation = ui.ctx().animate_bool_responsive(response.id, *self.value);
      let rect = rect.expand(0.0); // What it is?? TODO: We don't have it in theme

      let track_pos_min =
        Pos2::new(rect.min.x + 1.0 + (rect.width() / 2.0 * animation), rect.min.y + 1.0);
      let track_pos_max = Pos2::new(
        rect.max.x - 1.0 - rect.width() / 2.0 + (rect.width() / 2.0 * animation),
        rect.max.y - 1.0,
      );
      let track_rect = Rect::from_min_max(track_pos_min, track_pos_max);
      let track_rect = track_rect.expand(0.0);

      ui.painter().rect(
        rect,
        1.0,
        theme.colors.surface_container_low,
        Stroke::new(1.0, theme.colors.on_surface),
        egui::StrokeKind::Outside,
      );

      ui.painter().rect_filled(track_rect, 1.0, theme.colors.on_surface);
    }

    response
  }
}