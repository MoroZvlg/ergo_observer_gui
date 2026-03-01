use crate::state::AppState;
use crate::state::app_state::WindowType;
use crate::theme::{FontExt, Theme};
use crate::widgets::{EXIT_ICON, SETTINGS_ICON};
use eframe::emath::Align;
use egui::scroll_area::ScrollBarVisibility;
use egui::{Align2, Color32, Frame, Layout, Margin, RichText, Stroke, TextStyle, Ui, Vec2};

pub mod node_connect;
pub mod settings;

use crate::theme::fonts::heading_xs;
pub use node_connect::node_content;
pub use settings::settings_content;

pub struct Window;

impl Window {
  pub fn show(ctx: &egui::Context, state: &mut AppState) {
    let screen_high = 320.0_f32.max(ctx.content_rect().height());

    for window_type in state.windows_config.active_windows() {
      let window = egui::Window::new(window_type.to_string())
        .resizable(false)
        .interactable(false)
        .movable(true)
        .collapsible(false)
        .title_bar(false)
        .vscroll(false)
        .default_pos(ctx.content_rect().center() - 0.3 * screen_high * Vec2::Y)
        .min_height(screen_high * 0.2)
        .min_width(556.0)
        .default_height(screen_high * 0.4)
        .default_width(556.0)
        .frame(
          Frame::new()
            .inner_margin(0)
            .stroke(Stroke::new(0.0, Color32::TRANSPARENT))
            .corner_radius(0),
        )
        .pivot(Align2::CENTER_TOP);

      match window_type {
        WindowType::Settings => {
          window.show(ctx, |ui| {
            let response = WindowContent::new("Settings").show(ui, |ui| {
              settings_content(ui, state);
            });
            if response.closed {
              state.windows_config.close(WindowType::Settings);
            }
          });
        }
        WindowType::NodeConnect => {
          window.show(ctx, |ui| {
            let response = WindowContent::new("NodeConnect").show(ui, |ui| {
              node_content(ui, state);
            });
            if response.closed {
              state.windows_config.close(WindowType::NodeConnect);
            }
          });
        }
      };
    }
  }
}

pub struct WindowResponse {
  closed: bool,
}

pub struct WindowContent {
  pub title: String,
}

impl WindowContent {
  pub fn new(title: &str) -> Self {
    Self { title: title.to_string() }
  }

  pub fn show<R>(
    &mut self,
    ui: &mut Ui,
    add_contents: impl FnOnce(&mut Ui) -> R,
  ) -> WindowResponse {
    let theme = Theme::get_theme(&ui.ctx());
    let mut response = WindowResponse { closed: false };
    Frame::new()
      .inner_margin(Margin { top: 0, left: 0, right: 0, bottom: 3 })
      .stroke(Stroke::new(2.0, theme.colors.outline))
      .corner_radius(1)
      .show(ui, |ui| {
        ui.style_mut().spacing.item_spacing.y = 0.0;
        ui.vertical(|ui| {
          // Title
          Frame::new().fill(theme.colors.surface_container_high).inner_margin(5.0).show(ui, |ui| {
            ui.horizontal(|ui| {
              let high = ui.text_style_height(&heading_xs());
              ui.add(
                SETTINGS_ICON
                  .as_image()
                  .fit_to_exact_size(Vec2::new(high, high))
                  .tint(theme.colors.on_surface),
              );

              ui.label(RichText::new(&self.title).heading_xs().color(theme.colors.on_surface));

              ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                if ui.add(EXIT_ICON.fill(theme.colors.on_surface).as_button()).clicked() {
                  response.closed = true;
                }
              });
            });
            ui.set_width(ui.available_width());
          });

          // Body
          egui::ScrollArea::new([true, true])
            .scroll_bar_visibility(ScrollBarVisibility::AlwaysHidden)
            .show(ui, |ui| {
              Frame::new().fill(theme.colors.surface_container).inner_margin(5.0).show(ui, |ui| {
                add_contents(ui);

                ui.add_space(ui.available_height());
                ui.set_width(ui.available_width());
              });
            });
        });
      });
    response
  }
}
