use crate::theme::Theme;
use eframe::epaint::Color32;
use egui::{AtomLayout, AtomLayoutResponse, Atoms, Frame, IntoAtoms, Response, Sense, Stroke, Ui, Vec2, Widget};

pub struct Button<'a> {
  layout: AtomLayout<'a>,
  inner_margin: Vec2,
  outlined: bool,
}

impl<'a> Button<'a> {
  pub fn new(atoms: impl IntoAtoms<'a>) -> Self {
    Self {
      layout: AtomLayout::new(atoms.into_atoms()).sense(Sense::click()),
      inner_margin: Vec2::splat(2.0),
      outlined: false
    }
  }

  pub fn outlined(mut self) -> Self {
    self.outlined = true;
    self
  }

  fn atom_ui(self, ui: &mut Ui) -> AtomLayoutResponse {
    let theme = Theme::get_theme(&ui.ctx());

    let stroke = if self.outlined {
      Stroke::new(1.0, theme.colors.outline)
    } else {
      Stroke::NONE
    };
    let mut atoms_layout = self
      .layout
      .frame(
        Frame::new()
          .corner_radius(1.0)
          .inner_margin(self.inner_margin)
          .fill(Color32::TRANSPARENT)
          .stroke(stroke),
      )
      .allocate(ui);

    if ui.is_rect_visible(atoms_layout.response.rect) {
      if atoms_layout.response.hovered() {
        atoms_layout.frame = atoms_layout.frame.fill(theme.colors.surface_container_high);
      }
      atoms_layout.paint(ui)
    } else {
      AtomLayoutResponse::empty(atoms_layout.response)
    }
  }
}

impl<'a> Widget for Button<'a> {
  fn ui(self, ui: &mut Ui) -> Response {
    self.atom_ui(ui).response
  }
}
