use std::collections::HashMap;
use egui::{Color32, CornerRadius, Id, Pos2, Rect, RichText, Stroke, StrokeKind, TextStyle, TextWrapMode, Ui, Vec2, WidgetText};
use crate::widgets::canvas::actor::Actor;
use crate::widgets::canvas::ViewportTransform;

pub struct ActorGroup {
  actor_ids: Vec<Id>,
  stroke: Stroke,
  corner_radius: CornerRadius,
  text: WidgetText,
}

impl Default for ActorGroup {
  fn default() -> Self {
    Self {
      actor_ids: Vec::new(),
      stroke: Stroke::new(1.0, Color32::WHITE),
      corner_radius: CornerRadius::ZERO,
      text: RichText::new("Application").monospace().into(),
    }
  }
}

impl ActorGroup {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn with_label(mut self, label: impl Into<WidgetText>) -> Self {
    self.text = label.into();
    self
  }

  pub fn add_actor(&mut self, id: Id) {
    self.actor_ids.push(id);
  }

  pub fn draw(
    &mut self,
    actors: &HashMap<Id, Actor>,
    ui: &mut Ui,
    transform: &ViewportTransform,
  ) {
    let mut group_pos_min = Pos2::new(f32::INFINITY, f32::INFINITY);
    let mut group_pos_max = Pos2::new(f32::NEG_INFINITY, f32::NEG_INFINITY);
    self.actor_ids.iter().for_each(|id| {
      let actor = actors.get(id);
      if let Some(actor) = actor {
        let position = actor.position();
        let position_max = position + actor.size();
        if position.x < group_pos_min.x {
          group_pos_min.x = position.x
        }
        if position.y < group_pos_min.y {
          group_pos_min.y = position.y
        }
        if position_max.x > group_pos_max.x {
          group_pos_max.x = position_max.x
        }
        if position_max.y > group_pos_max.y {
          group_pos_max.y = position_max.y
        }
      }
    });

    ui.painter().rect_stroke(
      Rect::from_min_max(
        transform.to_screen(group_pos_min - Vec2::splat(20.0)),
        transform.to_screen(group_pos_max + Vec2::splat(20.0)),
      ),
      self.corner_radius,
      self.stroke,
      StrokeKind::Inside,
    );

    // TODO: figure out how to work with text. this one is shit
    let galley = self.text.clone().into_galley(
      ui,
      Some(TextWrapMode::Extend),
      f32::INFINITY,
      TextStyle::Body,
    );

    ui.painter().galley(
      transform.to_screen(group_pos_min - Vec2::splat(15.0)),
      galley,
      ui.visuals().text_color(),
    );
  }
}
