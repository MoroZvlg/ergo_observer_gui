use std::collections::HashSet;
use ewebsock::{WsEvent, WsMessage, WsReceiver, WsSender};
use crate::data::types::ApplicationTreeMessage;

pub enum ObserverEvent {
    Connected { node: String },
    ApplicationList { applications: Vec<String> },
    ApplicationTree(ApplicationTreeMessage),
    Disconnected,
}

enum Phase {
    WaitingIntro,
    WaitingConnectResult(String), // the CID we sent with the connect command
    Running,
}

pub struct ObserverWSClient {
    url: String,
    sender: Option<WsSender>,
    receiver: Option<WsReceiver>,
    phase: Phase,
    subscribed_apps: HashSet<String>,
    next_cid: u32,
    queued: Vec<ObserverEvent>,
    pending_node_name: String,
}

impl ObserverWSClient {
    pub fn new(url: &str) -> Self {
        let mut client = Self {
            url: url.to_string(),
            sender: None,
            receiver: None,
            phase: Phase::WaitingIntro,
            subscribed_apps: HashSet::new(),
            next_cid: 1,
            queued: Vec::new(),
            pending_node_name: String::new(),
        };
        client.connect();
        client
    }

    fn connect(&mut self) {
        log::info!("[observer] connecting to {}", self.url);
        match ewebsock::connect(&self.url, Default::default()) {
            Ok((sender, receiver)) => {
                self.sender = Some(sender);
                self.receiver = Some(receiver);
                self.phase = Phase::WaitingIntro;
                self.subscribed_apps.clear();
                self.next_cid = 1;
            }
            Err(e) => log::warn!("[observer] connect failed: {e}"),
        }
    }

    fn send(&mut self, value: serde_json::Value) {
        if let Some(sender) = &mut self.sender {
            sender.send(WsMessage::Text(value.to_string()));
        }
    }

    /// Send a connect command to switch to a different Ergo node.
    /// The same WS connection is reused; the observer server handles routing.
    pub fn switch_node(&mut self, name: &str) {
        let cid = self.next_cid.to_string();
        self.next_cid += 1;
        self.pending_node_name = name.to_string();
        self.subscribed_apps.clear();
        self.queued.clear();
        self.send(serde_json::json!({
            "Command": "connect",
            "Name": name,
            "CID": cid,
            "Args": {},
        }));
        self.phase = Phase::WaitingConnectResult(cid);
    }

    /// Call this every frame from AppState::process_events.
    pub fn poll(&mut self) {
        let messages: Vec<WsEvent> = match &self.receiver {
            Some(rx) => std::iter::from_fn(|| rx.try_recv()).collect(),
            None => return,
        };

        for event in messages {
            match event {
                WsEvent::Opened => {
                    log::debug!("[observer] WS opened, waiting for intro");
                }
                WsEvent::Message(WsMessage::Text(text)) => {
                    self.handle_text(text);
                }
                WsEvent::Closed => {
                    log::info!("[observer] WS closed, reconnecting...");
                    self.queued.push(ObserverEvent::Disconnected);
                    self.sender = None;
                    self.receiver = None;
                    self.connect();
                }
                WsEvent::Error(e) => {
                    log::warn!("[observer] WS error: {e}");
                    self.queued.push(ObserverEvent::Disconnected);
                    self.sender = None;
                    self.receiver = None;
                    self.connect();
                }
                _ => {}
            }
        }
    }

    pub fn drain_events(&mut self) -> Vec<ObserverEvent> {
        std::mem::take(&mut self.queued)
    }

    fn handle_text(&mut self, text: String) {
        let value: serde_json::Value = match serde_json::from_str(&text) {
            Ok(v) => v,
            Err(e) => { log::warn!("[observer] json parse error: {e}"); return; }
        };

        let has_event = value.get("Event").is_some();
        let has_cid = value.get("CID").is_some();

        match self.phase {
            Phase::WaitingIntro => {
                // Server sends intro: {Node: {Name, CRC32}, Peers: [...], Version: ...}
                if let Some(name) = value.pointer("/Node/Name").and_then(|v| v.as_str()) {
                    let node_name = name.to_string();
                    log::info!("[observer] got intro, node={node_name}");
                    self.pending_node_name = node_name.clone();
                    let cid = self.next_cid.to_string();
                    self.next_cid += 1;
                    self.send(serde_json::json!({
                        "Command": "connect",
                        "Name": node_name,
                        "CID": cid,
                    }));
                    self.phase = Phase::WaitingConnectResult(cid);
                }
            }

            Phase::WaitingConnectResult(ref expected_cid) => {
                // Ignore events; wait for our CID result
                if has_event || !has_cid { return; }
                let got_cid = value["CID"].as_str().unwrap_or("");
                if got_cid != expected_cid { return; }

                if let Some(err) = value.get("Error").and_then(|v| v.as_str()) {
                    log::warn!("[observer] connect rejected: {err}");
                    return;
                }

                let node = self.pending_node_name.clone();
                log::info!("[observer] connected to {node}, subscribing application_list");
                self.queued.push(ObserverEvent::Connected { node });

                let sub_cid = self.next_cid.to_string();
                self.next_cid += 1;
                self.send(serde_json::json!({
                    "Command": "subscribe",
                    "Name": "application_list",
                    "CID": sub_cid,
                }));
                self.phase = Phase::Running;
            }

            Phase::Running => {
                if !has_event { return; }
                let event_str = value["Event"].as_str().unwrap_or("");

                if event_str.contains("inspect_application_list") {
                    let apps = Self::parse_app_names(&value);

                    let new_apps: Vec<String> = apps.iter()
                        .filter(|a| !self.subscribed_apps.contains(*a))
                        .cloned()
                        .collect();

                    if !apps.is_empty() {
                        self.queued.push(ObserverEvent::ApplicationList { applications: apps });
                    }

                    for app in new_apps {
                        let cid = self.next_cid.to_string();
                        self.next_cid += 1;
                        log::info!("[observer] subscribing tree for {app}");
                        self.send(serde_json::json!({
                            "Command": "subscribe",
                            "Name": "application_tree",
                            "Args": { "Application": app, "Limit": 1000 },
                            "CID": cid,
                        }));
                        self.subscribed_apps.insert(app);
                    }
                } else if event_str.contains("inspect_application_tree") {
                    if let Some(msg_val) = value.get("Message") {
                        match serde_json::from_value::<ApplicationTreeMessage>(msg_val.clone()) {
                            Ok(tree_msg) => {
                                self.queued.push(ObserverEvent::ApplicationTree(tree_msg));
                            }
                            Err(e) => {
                                log::warn!("[observer] failed to parse tree: {e}");
                                log::debug!("[observer] raw: {}", msg_val);
                            }
                        }
                    }
                }
            }
        }
    }

    fn parse_app_names(value: &serde_json::Value) -> Vec<String> {
        value.pointer("/Message/Applications")
            .and_then(|v| v.as_object())
            .map(|obj| obj.keys().cloned().collect())
            .unwrap_or_default()
    }
}
