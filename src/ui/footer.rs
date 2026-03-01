use crate::state::AppState;
use crate::theme::Theme;
use eframe::egui::{TopBottomPanel, Context, Frame, Margin, Color32, Stroke, Layout, Align};
use egui::{Label, RichText};
use crate::state::app_state::ConnectionStatus;

pub struct Footer;

impl Footer {
  pub fn show(ctx: &Context, state: &AppState) {
    let theme = Theme::get_theme(&ctx);

    TopBottomPanel::bottom("footer")
      .show_separator_line(false)
      .frame(Frame::new().fill(theme.colors.surface).inner_margin(Margin::symmetric(10, 5)))
      .show(ctx, |ui| {
        ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
          ui.add(Label::new(RichText::new(&state.version).color(theme.colors.on_surface_variant).small()));
          ui.separator();
          let now = chrono::Local::now();
          ui.add(Label::new(RichText::new(now.format("%Y-%m-%d %H:%M:%S").to_string()).color(theme.colors.on_surface_variant).small()));

          // Right side
          ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            ui.separator();
            let status_color = match state.connection_status {
              ConnectionStatus::Connected => theme.colors.success,
              ConnectionStatus::Disconnected => theme.colors.error,
              ConnectionStatus::Connecting => theme.colors.on_surface_variant
            };
            ui.add(Label::new(RichText::new(state.connection_status.as_str()).color(status_color).small()));
            ui.separator();
            ui.add(Label::new(RichText::new(format!("FPS: {:.0}", state.fps)).color(theme.colors.on_surface_variant).small()));

          });
        });
      });
  }
}
