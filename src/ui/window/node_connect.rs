use crate::state::AppState;
use crate::state::app_state::WindowType;
use crate::theme::Theme;
use crate::widgets::Button;
use eframe::emath::{Align, Vec2};
use egui::{Color32, Grid, Key, Layout, RichText, TextEdit, TextStyle, Ui};

pub fn node_content(ui: &mut Ui, state: &mut AppState) {
  let theme = Theme::get_theme(ui.ctx());

  egui::Frame::new().show(ui, |ui| {
    let width = ui.available_width();
    Grid::new("node_connect")
      .spacing(Vec2::ZERO)
      .min_row_height(0.0)
      .min_col_width(0.0)
      .num_columns(2)
      .show(ui, |ui| {
        let response = ui.add(
          TextEdit::singleline(&mut state.node_name_input)
            .hint_text(format!("node_naame@localhost"))
            .font(TextStyle::Button)
            .clip_text(true)
            .min_size(Vec2::new(width - 80.0, ui.available_height()))
            .margin(Vec2::splat(3.0))
            .text_color(theme.colors.on_surface)
            .background_color(theme.colors.surface_dim),
        );
        let enter_pressed = response.lost_focus() && ui.input(|i| i.key_pressed(Key::Enter));

        ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
          let response = ui.add(
            Button::new(RichText::new("Connect").color(theme.colors.on_surface_variant)).outlined(),
          );

          if enter_pressed || response.clicked() {
            let name = state.node_name_input.clone();
            state.switch_node(&name);
          }
        });

        ui.end_row();
      });
  });

}
