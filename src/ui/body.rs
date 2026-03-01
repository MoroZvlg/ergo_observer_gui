use crate::state::AppState;
use crate::theme::Theme;
use crate::widgets::Canvas;
use egui::{Frame, Stroke};

pub struct Body;

impl Body {
  pub fn show(ctx: &egui::Context, state: &mut AppState) {
    let theme = Theme::get_theme(ctx);

    egui::CentralPanel::default()
      .frame(Frame::new().fill(theme.colors.surface).inner_margin(0.0))
      .show(ctx, |ui| {
        let theme = Theme::get_theme(ctx);
        Canvas::new()
          .bg_color(theme.colors.surface)
          .bg_stroke(Stroke::new(1.0, theme.colors.outline))
          .grid_color(theme.colors.outline)
          .show(ui, &mut state.canvas);
      });
  }
}
