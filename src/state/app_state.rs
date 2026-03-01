use std::collections::HashMap;
use std::fmt::Display;
use egui::Pos2;
use log::info;
use crate::data::{ObserverWSClient, ObserverEvent, Tree};
use crate::data::tree::{ROOT_PID, NODE_W, H_GAP};
use crate::theme::Theme;
use crate::widgets::CanvasDrawings;

#[derive(Clone, Debug)]
pub enum ConnectionStatus {
  Connected,
  Disconnected,
  Connecting,
}

impl ConnectionStatus {
  pub fn as_str(&self) -> &str {
    match self {
      ConnectionStatus::Connected => "● Connected",
      ConnectionStatus::Disconnected => "● Disconnected",
      ConnectionStatus::Connecting => "◌ Connecting...",
    }
  }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub enum WindowType {
  Settings,
  NodeConnect
}

impl Display for WindowType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      WindowType::Settings => write!(f, "Settings"),
      WindowType::NodeConnect => write!(f, "Node Connect"),
    }
  }
}

impl WindowType {
  const ALL: &'static [WindowType] = &[WindowType::Settings, WindowType::NodeConnect];

  fn default_map() -> HashMap<WindowType, bool> {
    Self::ALL.iter().map(|&window_type| (window_type, false)).collect()
  }
}

pub struct WindowsConfig {
  windows: HashMap<WindowType, bool>,
}

impl WindowsConfig {
  pub fn active_windows(&self) -> Vec<WindowType> {
    self.windows.iter().filter(|(_, opened)| **opened).map(|(window_type, _)| *window_type).collect()
  }

  pub fn open(&mut self, window_type: WindowType) {
    self.windows.insert(window_type, true).unwrap();
  }

  pub fn close(&mut self, window_type: WindowType) {
    self.windows.insert(window_type, false).unwrap();
  }
}

impl Default for WindowsConfig {
  fn default() -> Self {
    Self { windows: WindowType::default_map() }
  }
}

pub struct AppState {
  pub version: String,
  pub connection_status: ConnectionStatus,
  pub fps: f32,
  pub theme: Theme,
  pub windows_config: WindowsConfig,

  pub app_trees: HashMap<String, Tree>,
  pub canvas: CanvasDrawings,

  pub api_ws: ObserverWSClient,
  pub node_name_input: String,
}

impl AppState {
  pub fn new() -> Self {
    Self {
      version: "0.1.0".to_string(),
      connection_status: ConnectionStatus::Disconnected,
      fps: 60.0,
      theme: Theme::default(),
      windows_config: Default::default(),
      app_trees: HashMap::new(),
      canvas: CanvasDrawings::new(),
      api_ws: ObserverWSClient::new("ws://localhost:9911/ws"),
      node_name_input: String::new(),
    }
  }

  pub fn switch_node(&mut self, name: &str) {
    self.connection_status = ConnectionStatus::Connecting;
    self.app_trees.clear();
    self.canvas = CanvasDrawings::new();
    self.api_ws.switch_node(name);
  }

  fn rebuild_canvas(app_trees: &HashMap<String, Tree>) -> CanvasDrawings {
    const APP_GAP: f32 = H_GAP * 6.0;

    let mut canvas = CanvasDrawings::new();
    let mut current_x = 0.0f32;

    let mut sorted_apps: Vec<&String> = app_trees.keys().collect();
    sorted_apps.sort();

    for app_name in sorted_apps {
      let tree = &app_trees[app_name];
      let right_edge = canvas.add_tree(tree, Pos2::new(current_x, 0.0), app_name);
      current_x = right_edge + APP_GAP;
    }

    canvas
  }

  pub fn process_events(&mut self) {
    self.api_ws.poll();

    let mut trees_changed = false;

    for event in self.api_ws.drain_events() {
      match event {
        ObserverEvent::Connected { node } => {
          info!("[observer] connected to node: {node}");
          self.connection_status = ConnectionStatus::Connected;
          self.node_name_input = node;
          self.app_trees.clear();
          trees_changed = true;
        }
        ObserverEvent::Disconnected => {
          info!("[observer] disconnected");
          self.connection_status = ConnectionStatus::Disconnected;
        }
        ObserverEvent::ApplicationList { applications } => {
          info!("[observer] application list: {:?}", applications);
        }
        ObserverEvent::ApplicationTree(msg) => {
          let app_name = msg.application.clone();
          let all_pids: std::collections::HashSet<u64> =
            msg.processes.iter().map(|p| p.pid.id).collect();
          let mut tree = Tree::new();
          for mut process in msg.processes {
            if !all_pids.contains(&process.parent.id) {
              process.parent.id = ROOT_PID;
            }
            tree.add_node(process, true);
          }
          self.app_trees.insert(app_name, tree);
          trees_changed = true;
        }
      }
    }

    if trees_changed {
      self.canvas = Self::rebuild_canvas(&self.app_trees);
    }
  }

  pub fn toggle_theme(&mut self) {
    self.theme = if self.theme.name == "Dark" { Theme::light() } else { Theme::dark() };
  }
}

impl Default for AppState {
  fn default() -> Self {
    Self::new()
  }
}
