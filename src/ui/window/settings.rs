use crate::state::AppState;
use crate::theme::{FontExt, Theme};
use crate::widgets::{Toggle};
use eframe::emath::Align;
use egui::{Grid, Layout, RichText, Ui};

pub fn settings_content(ui: &mut Ui, state: &mut AppState) {
  egui::Frame::new().show(ui, |ui| {
    Grid::new("theme").num_columns(4).spacing([40.0, 2.0]).show(
      ui,
      |ui| {
        ui.set_width(ui.available_width());
        ui.label("");
        ui.label(styled_text(&state.theme.name, &state.theme));
        let mut toggle_value = state.theme.name == "Dark";
        let response = Toggle::new(&mut toggle_value).show(ui);
        if response.clicked() {
          state.toggle_theme();
        }
        ui.end_row();

        ui.with_layout(Layout::top_down(Align::Center), |ui| {
          ui.label(styled_text_label("Accent", &state.theme))
        });
        ui.end_row();

        ui.label(styled_text("Primary", &state.theme));
        ui.color_edit_button_srgba(&mut state.theme.colors.primary);
        ui.label(styled_text("On Primary", &state.theme));
        ui.color_edit_button_srgba(&mut state.theme.colors.on_primary);
        ui.end_row();

        ui.label(styled_text("Primary Container", &state.theme));
        ui.color_edit_button_srgba(&mut state.theme.colors.primary_container);
        ui.label(styled_text("On Primary Container", &state.theme));
        ui.color_edit_button_srgba(&mut state.theme.colors.on_primary_container);
        ui.end_row();

        ui.label(styled_text("Secondary", &state.theme));
        ui.color_edit_button_srgba(&mut state.theme.colors.secondary);
        ui.label(styled_text("On Secondary", &state.theme));
        ui.color_edit_button_srgba(&mut state.theme.colors.on_secondary);
        ui.end_row();

        ui.label(styled_text("Secondary Container", &state.theme));
        ui.color_edit_button_srgba(&mut state.theme.colors.secondary_container);
        ui.label(styled_text("On Secondary Container", &state.theme));
        ui.color_edit_button_srgba(&mut state.theme.colors.on_secondary_container);
        ui.end_row();

        ui.with_layout(Layout::top_down(Align::Center), |ui| {
          ui.label(styled_text_label("Surface", &state.theme))
        });
        ui.end_row();

        ui.label(styled_text("Surface Dim", &state.theme));
        ui.color_edit_button_srgba(&mut state.theme.colors.surface_dim);
        ui.label(styled_text("On Surface", &state.theme));
        ui.color_edit_button_srgba(&mut state.theme.colors.on_surface);
        ui.end_row();

        ui.label(styled_text("Surface", &state.theme));
        ui.color_edit_button_srgba(&mut state.theme.colors.surface);
        ui.label(styled_text("On Surface Variant", &state.theme));
        ui.color_edit_button_srgba(&mut state.theme.colors.on_surface_variant);
        ui.end_row();

        ui.label(styled_text("Surface Bright", &state.theme));
        ui.color_edit_button_srgba(&mut state.theme.colors.surface_bright);
        ui.label(styled_text("Outline", &state.theme));
        ui.color_edit_button_srgba(&mut state.theme.colors.outline);
        ui.end_row();

        ui.label(styled_text("Surface Container Low", &state.theme));
        ui.color_edit_button_srgba(&mut state.theme.colors.surface_container_low);
        ui.label(styled_text("Outline Variant", &state.theme));
        ui.color_edit_button_srgba(&mut state.theme.colors.outline_variant);
        ui.end_row();

        ui.label(styled_text("Surface Container", &state.theme));
        ui.color_edit_button_srgba(&mut state.theme.colors.surface_container);
        ui.end_row();

        ui.label(styled_text("Surface Container High", &state.theme));
        ui.color_edit_button_srgba(&mut state.theme.colors.surface_container_high);
        ui.end_row();

        ui.with_layout(Layout::top_down(Align::Center), |ui| {
          ui.label(styled_text_label("Success", &state.theme))
        });
        ui.end_row();

        ui.label(styled_text("Success", &state.theme));
        ui.color_edit_button_srgba(&mut state.theme.colors.success);
        ui.label(styled_text("On Success", &state.theme));
        ui.color_edit_button_srgba(&mut state.theme.colors.on_success);
        ui.end_row();

        ui.label(styled_text("Success Container", &state.theme));
        ui.color_edit_button_srgba(&mut state.theme.colors.success_container);
        ui.label(styled_text("On Success Container", &state.theme));
        ui.color_edit_button_srgba(&mut state.theme.colors.on_success_container);
        ui.end_row();

        ui.with_layout(Layout::top_down(Align::Center), |ui| {
          ui.label(styled_text_label("Error", &state.theme))
        });
        ui.end_row();

        ui.label(styled_text("Error", &state.theme));
        ui.color_edit_button_srgba(&mut state.theme.colors.error);
        ui.label(styled_text("On Error", &state.theme));
        ui.color_edit_button_srgba(&mut state.theme.colors.on_error);
        ui.end_row();

        ui.label(styled_text("Error Container", &state.theme));
        ui.color_edit_button_srgba(&mut state.theme.colors.error_container);
        ui.label(styled_text("On Error Container", &state.theme));
        ui.color_edit_button_srgba(&mut state.theme.colors.on_error_container);
        ui.end_row();
      },
    );
  });
}

fn styled_text(text: &str, theme: &Theme) -> RichText {
  RichText::new(text).color(theme.colors.on_surface_variant).monospace()
}
fn styled_text_label(text: &str, theme: &Theme) -> RichText {
  RichText::new(text).heading_xs().color(theme.colors.on_surface)
}
