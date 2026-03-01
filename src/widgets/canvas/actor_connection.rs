use std::collections::HashMap;
use egui::emath::Rot2;
use egui::{Color32, Id, Pos2, Stroke, Ui, Vec2};
use crate::theme::ColorExt;
use crate::widgets::canvas::actor::Actor;
use crate::widgets::canvas::transform::ViewportTransform;

#[derive(Clone)]
pub enum PortType {
  Top,
  Bottom,
  Left,
  Right,
}

pub struct ActorConnection {
  from: Id,
  to: Id,
  stroke: Stroke,
}

impl ActorConnection {
  pub fn new(from: Id, to: Id) -> Self {
    Self { from, to, stroke: Stroke::new(1.0, Color32::GRAY.gamma_multiply(0.5)) }
  }

  pub fn draw(
    &mut self,
    actors: &HashMap<Id, Actor>,
    ui: &mut Ui,
    transform: &ViewportTransform,
  ) {
    if actors.get(&self.from).is_none() || actors.get(&self.to).is_none() {
      return
    }

    let (pos_from, pos_to, port_from, port_to) = self.positions(actors);

    let mid_x = (pos_from.x + pos_to.x) / 2.0;
    let mid_y = (pos_from.y + pos_to.y) / 2.0;
    let points: [Pos2; 4];
    match port_from {
      PortType::Top | PortType::Bottom => {
        points = [pos_from, Pos2::new(pos_from.x, mid_y), Pos2::new(pos_to.x, mid_y), pos_to];
      }
      PortType::Left | PortType::Right => {
        points = [pos_from, Pos2::new(mid_x, pos_from.y), Pos2::new(mid_x, pos_to.y), pos_to];
      }
    }

    for pair in points.windows(2) {
      ui.painter().line_segment(
        [transform.to_screen(pair[0]), transform.to_screen(pair[1])],
        self.stroke,
      );
    }

    let rot = Rot2::from_angle(std::f32::consts::TAU / 10.0);
    let arrow_size = match port_to {
      PortType::Top => Vec2::new(0.0, 10.0),
      PortType::Bottom => Vec2::new(0.0, -10.0),
      PortType::Left => Vec2::new(10.0, 0.0),
      PortType::Right => Vec2::new(-10.0, 0.0),
    };
    ui.painter().line_segment(
      [
        transform.to_screen(points[3]),
        transform.to_screen(points[3] - (rot * arrow_size)),
      ],
      self.stroke,
    );
    ui.painter().line_segment(
      [
        transform.to_screen(points[3]),
        transform.to_screen(points[3] - (rot.inverse() * arrow_size)),
      ],
      self.stroke,
    );
  }

  fn positions(
    &self,
    actors: &HashMap<Id, Actor>,
  ) -> (Pos2, Pos2, PortType, PortType) {
    let from = actors.get(&self.from).unwrap();
    let to = actors.get(&self.to).unwrap();

    let dx = to.position().x - from.position().x;
    let dy = from.position().y - to.position().y;

    // If we want to connect actors on right and left
    // let (port_from, port_to) = if dy.abs() > dx.abs() {
    //   if dy > 0.0 {
    //     (PortType::Top, PortType::Bottom)
    //   } else {
    //     (PortType::Bottom, PortType::Top)
    //   }
    // } else if dx > 0.0 {
    //   (PortType::Right, PortType::Left)
    // } else {
    //   (PortType::Left, PortType::Right)
    // };
    let (port_from, port_to) = if dy > 0.0 {
      (PortType::Top, PortType::Bottom)
    } else {
      (PortType::Bottom, PortType::Top)
    };

    (
      Self::port_position(from.position(), from.size(), &port_from),
      Self::port_position(to.position(), to.size(), &port_to),
      port_from,
      port_to,
    )
  }

  fn port_position(actor_pos: Pos2, actor_size: Vec2, port_type: &PortType) -> Pos2 {
    match port_type {
      PortType::Top => Pos2::new(actor_pos.x + actor_size.x / 2.0, actor_pos.y),
      PortType::Bottom => Pos2::new(actor_pos.x + actor_size.x / 2.0, actor_pos.y + actor_size.y),
      PortType::Right => Pos2::new(actor_pos.x + actor_size.x, actor_pos.y + actor_size.y / 2.0),
      PortType::Left => Pos2::new(actor_pos.x, actor_pos.y + actor_size.y / 2.0),
    }
  }
}
