use std::collections::HashMap;
use eframe::epaint::Vec2;
use egui::{Id, Pos2, RichText, Ui};
use crate::data::tree::{Node, Tree, ROOT_PID, NODE_W};
use crate::widgets::canvas::actor::Actor;
use crate::widgets::canvas::actor_connection::ActorConnection;
use crate::widgets::canvas::actor_group::ActorGroup;
use super::transform::ViewportTransform;

pub struct CanvasDrawings {
  processes: HashMap<Id, Actor>,
  connections: Vec<ActorConnection>,
  applications: Vec<ActorGroup>,
}

impl Default for CanvasDrawings {
  fn default() -> Self {
    Self { processes: HashMap::new(), connections: Vec::new(), applications: Vec::new() }
  }
}

impl CanvasDrawings {
  pub fn new() -> Self {
    Self::default()
  }

  /// Add all actors from `tree` onto this canvas, translated by `offset`.
  /// Returns the right edge X of the added content (useful for placing the next app).
  pub fn add_tree(&mut self, tree: &Tree, offset: Pos2, label: &str) -> f32 {
    let positions = tree.compute_layout();

    let mut group = ActorGroup::new().with_label(RichText::new(label).monospace());
    let mut max_x = offset.x;

    for (pid, node) in &tree.all_processes {
      if *pid == ROOT_PID { continue; }

      let actor_id = Id::new(*pid);
      let raw_pos = *positions.get(pid).unwrap_or(&Pos2::ZERO);
      let pos = Pos2::new(raw_pos.x + offset.x, raw_pos.y + offset.y);

      let mut actor = Self::actor_from_node(node);
      actor.position = pos;
      max_x = max_x.max(pos.x);

      self.processes.insert(actor_id, actor);
      group.add_actor(actor_id);
    }

    for (pid, node) in &tree.all_processes {
      if *pid == ROOT_PID { continue; }
      let actor_id = Id::new(*pid);
      for child_pid in &node.childrens {
        self.connections.push(ActorConnection::new(actor_id, Id::new(*child_pid)));
      }
    }

    self.applications.push(group);
    max_x + NODE_W
  }

  fn actor_from_node(node: &Node) -> Actor {
    let mut actor = Actor::new(
      node.pid.to_string(),
      node.name.clone(),
      node.behavior.clone(),
    );
    actor.msg_in = node.messages_in;
    actor.msg_out = node.messages_out;
    actor.msg_mailbox = node.messages_mailbox;
    actor.log_level = node.log_level.as_str().to_string();
    actor.runtime = node.running_time;
    actor.status = node.state.as_str().to_string();
    actor
  }

  pub fn draw(&mut self, ui: &mut Ui, transform: &ViewportTransform) {
    ui.scope(|ui| {
      ui.spacing_mut().item_spacing = Vec2::ZERO;
      ui.spacing_mut().menu_spacing = 0.0;
      ui.spacing_mut().icon_spacing = 0.0;
      ui.style_mut().text_styles.iter_mut().for_each(|(_style, font_id)| {
        font_id.size = transform.to_screen(font_id.size)
      });

      for actor in self.processes.values_mut() {
        actor.draw(ui, transform);
      }

      for connection in &mut self.connections {
        connection.draw(&self.processes, ui, transform)
      }

      for application in &mut self.applications {
        application.draw(&self.processes, ui, transform);
      }
    });
  }
}
