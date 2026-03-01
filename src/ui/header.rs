use crate::state::AppState;
use crate::state::app_state::WindowType;
use crate::theme::Theme;
use crate::widgets::{Button, SETTINGS_ICON};
use eframe::egui::{
  Align, Context, Frame, Key, Label, Layout, Margin, RichText, TextEdit, TextStyle, TopBottomPanel,
};

pub struct Header;

impl Header {
  pub fn show(ctx: &Context, state: &mut AppState) {
    let theme = Theme::get_theme(&ctx);
    TopBottomPanel::top("header")
      .show_separator_line(false)
      .frame(Frame::new().fill(theme.colors.surface_dim).inner_margin(Margin::symmetric(10, 4)))
      .show(ctx, |ui| {
        ui.horizontal_centered(|ui| {
          ui.set_height(25.0);
          ui.with_layout(Layout::left_to_right(Align::Max), |ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.add(Label::new(RichText::new("Observer.").color(theme.colors.on_surface).heading()));
            ui.add(Label::new(RichText::new("<tools>").color(theme.colors.primary).monospace()));
          });

          ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            if ui
              .add(SETTINGS_ICON.max_size(24.0).fill(theme.colors.on_surface).as_button())
              .clicked()
            {
              state.windows_config.open(WindowType::Settings);
            }
            if ui
              .add(
                Button::new(RichText::new("Nodes").color(theme.colors.on_surface_variant))
                  .outlined(),
              )
              .clicked()
            {
              state.windows_config.open(WindowType::NodeConnect);
            }
          })
        })
      });
  }
}
