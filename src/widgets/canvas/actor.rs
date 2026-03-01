use crate::theme::{FontExt, Theme};
use crate::widgets::canvas::transform::ViewportTransform;
use egui::{Margin, Rect, Align, Atom, AtomLayout, Atoms, Color32, CornerRadius, Direction, Frame, Grid, Id, IntoAtoms, Layout, Pos2, RichText, Sense, Stroke, StrokeKind, Ui, UiBuilder, Vec2};
use log::info;

const HEADER_HEIGHT: f32 = 25.0;
const FOOTER_HEIGHT: f32 = 25.0;

pub struct Actor {
  pub pid: String,
  pub name: String,
  pub behavior: String,

  pub msg_in: u64,
  pub msg_out: u64,
  pub msg_mailbox: u64,
  pub log_level: String,

  pub runtime: u64, // ms
  pub status: String,

  pub stats_opened: bool,
  pub color: Color32,
  pub stroke: Stroke,
  pub position: Pos2,
  pub size: Vec2,
}

impl Actor {
  pub fn new(id: String, name: String, kind: String) -> Self {
    Self {
      pid: id,
      name,
      behavior: kind,

      msg_in: 10,
      msg_out: 5,
      msg_mailbox: 1,
      log_level: "info".to_string(),

      runtime: 1771180495000,
      status: "Running".to_string(),

      stats_opened: false,
      color: Color32::WHITE,
      stroke: Stroke::new(1.0, Color32::WHITE),
      position: Pos2::new(0.0, 0.0),
      size: Vec2::splat(30.0),
    }
  }

  pub fn draw(&mut self, ui: &mut Ui, transform: &ViewportTransform) {
    let desired_size = self.size();
    let desired_rect = Rect::from_min_size(self.position, desired_size);
    let theme = Theme::get_theme(&ui.ctx());
    let viewport_rect = transform.to_screen(desired_rect);

    if ui.is_rect_visible(viewport_rect) {
      let response = CardLayout::new(desired_rect).stroke(Stroke::new(1.0, theme.colors.outline)).show(ui, transform, &theme, |s, ui| {
        s.header(ui, |ui| {
          ui.with_layout(Layout::top_down_justified(Align::Center), |ui| {
            ui.label(RichText::new(self.pid.clone()).small().color(theme.colors.on_surface));
          });
        });

        s.body(ui, |ui| {
          ui.with_layout(Layout::top_down_justified(Align::Center), |ui| {
            ui.label(RichText::new(self.name.clone()).small().color(theme.colors.on_surface));
          });
          let text = RichText::new("Stats").color(theme.colors.on_surface).small();
          StatsToggle::new(&mut self.stats_opened, text).show(ui, &theme, |ui| {
            Frame::new().inner_margin(Margin::symmetric(5, 0)).show(ui, |ui| {
              Grid::new("actor_stats")
                .spacing(Vec2::ZERO)
                .min_row_height(0.0)
                .min_col_width(0.0)
                .num_columns(2)
                .show(ui, |ui| {
                  let label = |text: &str| RichText::new(text).small().color(theme.colors.on_surface);
                  let value = |text: String| RichText::new(text).small().color(theme.colors.on_surface);

                  ui.label(label("Msg IN"));
                  ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                    ui.label(value(self.msg_in.to_string()));
                  });
                  ui.end_row();

                  ui.label(label("Msg OUT"));
                  ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                    ui.label(value(self.msg_out.to_string()));
                  });
                  ui.end_row();

                  ui.label(label("Msg Mailbox"));
                  ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                    ui.label(value(self.msg_mailbox.to_string()));
                  });
                  ui.end_row();

                  ui.label(label("Log Level"));
                  ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                    ui.label(value(self.log_level.clone()));
                  });
                  ui.end_row();
                });
            });

          });
        });

        s.footer(ui, |ui| {
          ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
            ui.label(
              RichText::new(format!("{}", self.runtime)).text_xs().color(theme.colors.on_surface),
            );
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
              ui.label(RichText::new(self.status.clone()).text_xs().color(theme.colors.on_surface));
            });
          });
        });
      });

      if response.dragged {
        self.position += transform.to_canvas(response.drag_diff);
      }
    }
  }

  pub fn position(&self) -> Pos2 {
    self.position
  }

  pub fn size(&self) -> Vec2 {
    let mut desired_size = Vec2::new(200.0, 110.0);
    if self.stats_opened {
      desired_size += Vec2::new(0.0, 60.0);
    }
    desired_size
  }
}

struct CardResponse {
  pub dragged: bool,
  pub drag_diff: Vec2,
}

struct CardLayout {
  rect: Rect,
  stroke: Stroke
}

impl CardLayout {
  fn new(rect: Rect) -> Self {
    Self { rect, stroke: Stroke::NONE }
  }
  fn stroke(mut self, stroke: Stroke) -> Self {
    self.stroke = stroke;
    self
  }

  fn show(
    self,
    ui: &mut Ui,
    transform: &ViewportTransform,
    theme: &Theme,
    content: impl FnOnce(&mut CardSections, &mut Ui),
  ) -> CardResponse {
    let viewport_rect = transform.to_screen(self.rect);
    let response = ui.allocate_rect(viewport_rect, Sense::drag());

    let mut card_ui =
      ui.new_child(UiBuilder::new().max_rect(viewport_rect).layout(Layout::top_down(Align::Min)));

    Frame::new()
      .fill(theme.colors.surface_container)
      .stroke(self.stroke)
      .corner_radius(CornerRadius::same(1))
      .inner_margin(0.0)
      .outer_margin(0.0)
      .show(&mut card_ui, |ui| {
        ui.set_width(ui.available_width());
        ui.set_height(ui.available_height());

        let mut sections = CardSections { rect: self.rect, transform, theme };
        content(&mut sections, ui);
      });

    let mut card_resp = CardResponse { dragged: false, drag_diff: Vec2::ZERO };
    if response.dragged() {
      card_resp.dragged = true;
      card_resp.drag_diff = response.drag_delta();
    }
    card_resp
  }
}

struct CardSections<'a> {
  rect: Rect,
  transform: &'a ViewportTransform,
  theme: &'a Theme,
}

impl CardSections<'_> {
  fn header(&self, ui: &mut Ui, content: impl FnOnce(&mut Ui)) {
    let header_rect = self.transform.to_screen(Rect::from_min_max(
      self.rect.min,
      Pos2::new(self.rect.max.x, self.rect.min.y + HEADER_HEIGHT - 1.0),
    ));
    let mut header_ui = ui.new_child(
      UiBuilder::new()
        .max_rect(header_rect)
        .layout(Layout::centered_and_justified(Direction::TopDown)),
    );
    self.section_frame().show(&mut header_ui, content);
    ui.painter().hline(
      header_rect.x_range().shrink(4.0),
      header_rect.max.y,
      Stroke::new(1.0, self.theme.colors.outline),
    );
  }

  fn body(&self, ui: &mut Ui, content: impl FnOnce(&mut Ui)) {
    let body_rect = self.transform.to_screen(Rect::from_min_max(
      Pos2::new(self.rect.min.x, self.rect.min.y + HEADER_HEIGHT),
      Pos2::new(self.rect.max.x, self.rect.max.y - FOOTER_HEIGHT),
    ));
    let mut body_ui =
      ui.new_child(UiBuilder::new().max_rect(body_rect).layout(Layout::top_down(Align::Center)));
    self.section_frame().show(&mut body_ui, content);
  }

  fn footer(&self, ui: &mut Ui, content: impl FnOnce(&mut Ui)) {
    let footer_rect = self.transform.to_screen(Rect::from_min_max(
      Pos2::new(self.rect.min.x, self.rect.max.y - FOOTER_HEIGHT + 1.0),
      self.rect.max,
    ));
    let mut footer_ui = ui.new_child(
      UiBuilder::new()
        .max_rect(footer_rect)
        .layout(Layout::centered_and_justified(Direction::BottomUp)),
    );
    ui.painter().hline(
      footer_rect.x_range().shrink(4.0),
      footer_rect.min.y,
      Stroke::new(1.0, self.theme.colors.outline),
    );
    self.section_frame().show(&mut footer_ui, content);
  }

  fn section_frame(&self) -> Frame {
    Frame::new()
      .fill(Color32::TRANSPARENT)
      .stroke(Stroke::NONE)
      .corner_radius(CornerRadius::same(1))
      .inner_margin(self.transform.to_screen(5.0))
      .outer_margin(0.0)
  }
}

struct StatsToggle<'a> {
  text: Atoms<'a>,
  opened: &'a mut bool,
}

impl<'a> StatsToggle<'a> {
  fn new(opened: &'a mut bool, atoms: impl IntoAtoms<'a>) -> Self {
    Self { text: atoms.into_atoms(), opened }
  }

  fn show(self, ui: &mut Ui, theme: &Theme, content: impl FnOnce(&mut Ui)) {
    ui.spacing_mut().item_spacing = Vec2::ZERO;
    let avail_rect = ui.available_rect_before_wrap();
    let toggle_width = avail_rect.x_range().span();
    let arrow = if *self.opened { "▼" } else { "▶" };

    let layout = AtomLayout::new((self.text, Atom::grow(), arrow))
      .sense(Sense::click())
      .min_size(Vec2::new(toggle_width, 0.0));

    let mut prepare = layout.allocate(ui);
    if prepare.response.clicked() {
      *self.opened = !*self.opened;
      prepare.response.mark_changed();
    }

    let fill_color = if prepare.response.hovered() {
      theme.colors.surface_container_high
    } else {
      Color32::TRANSPARENT
    };

    if ui.is_rect_visible(prepare.response.rect) {
      ui.painter().rect(
        prepare.response.rect,
        CornerRadius::same(1),
        fill_color,
        Stroke::NONE,
        StrokeKind::Inside,
      );

      if *self.opened {
        let content_rect = Rect::from_min_max(
          Pos2::new(avail_rect.min.x, avail_rect.min.y + prepare.response.rect.y_range().span()),
          avail_rect.max,
        );
        let mut content_ui = ui.new_child(
          UiBuilder::new().max_rect(content_rect).layout(Layout::top_down(Align::Center)),
        );
        content(&mut content_ui);
      }
      prepare.paint(ui);
    }
  }
}
