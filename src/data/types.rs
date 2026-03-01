use serde::Deserialize;

/// gen.PID serializes as string: "<CRC32.int32(ID>>32).int32(ID)>"
#[derive(Debug, Clone)]
pub struct ErgoPid {
    pub node: String, // CRC32 of node name
    pub id: u64,
    pub creation: i64,
}

impl<'de> Deserialize<'de> for ErgoPid {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = String::deserialize(d)?;
        // Format: <CRC32.high.low>
        let inner = s.trim_start_matches('<').trim_end_matches('>');
        let parts: Vec<&str> = inner.splitn(3, '.').collect();
        if parts.len() != 3 {
            return Err(serde::de::Error::custom(format!("invalid PID: {s}")));
        }
        let high: i64 = parts[1].parse().map_err(serde::de::Error::custom)?;
        let low: i64 = parts[2].parse().map_err(serde::de::Error::custom)?;
        // Reconstruct: high = int32(ID>>32), low = int32(ID)
        let id = ((high as i32 as u32 as u64) << 32) | (low as i32 as u32 as u64);
        Ok(ErgoPid { node: parts[0].to_string(), id, creation: 0 })
    }
}

/// gen.ProcessState serializes as a lowercase string on the wire
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessState {
    Init,
    Sleep,
    Running,
    WaitResponse,
    Terminated,
    Zombee,
}

impl ProcessState {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Init => "init",
            Self::Sleep => "sleep",
            Self::Running => "running",
            Self::WaitResponse => "wait_response",
            Self::Terminated => "terminated",
            Self::Zombee => "zombee",
        }
    }
}

impl<'de> Deserialize<'de> for ProcessState {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = String::deserialize(d)?;
        match s.as_str() {
            "init" => Ok(Self::Init),
            "sleep" => Ok(Self::Sleep),
            "running" => Ok(Self::Running),
            "wait_response" | "waitresponse" => Ok(Self::WaitResponse),
            "terminated" => Ok(Self::Terminated),
            "zombee" => Ok(Self::Zombee),
            other => Err(serde::de::Error::custom(format!("unknown ProcessState: {other}"))),
        }
    }
}

/// gen.LogLevel serializes as a lowercase string on the wire
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    System,
    Trace,
    Debug,
    Default,
    Info,
    Warning,
    LogError,
    Panic,
    Disabled,
}

impl LogLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::System => "system",
            Self::Trace => "trace",
            Self::Debug => "debug",
            Self::Default => "default",
            Self::Info => "info",
            Self::Warning => "warning",
            Self::LogError => "error",
            Self::Panic => "panic",
            Self::Disabled => "disabled",
        }
    }
}

impl<'de> Deserialize<'de> for LogLevel {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = String::deserialize(d)?;
        match s.as_str() {
            "system" => Ok(Self::System),
            "trace" => Ok(Self::Trace),
            "debug" => Ok(Self::Debug),
            "default" => Ok(Self::Default),
            "info" => Ok(Self::Info),
            "warning" => Ok(Self::Warning),
            "error" => Ok(Self::LogError),
            "panic" => Ok(Self::Panic),
            "disabled" => Ok(Self::Disabled),
            other => Err(serde::de::Error::custom(format!("unknown LogLevel: {other}"))),
        }
    }
}

/// gen.ProcessShortInfo — all PID fields are strings on the wire
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ProcessShortInfo {
    #[serde(rename = "PID")]
    pub pid: ErgoPid,
    pub name: String,
    pub application: String,
    pub behavior: String,
    pub messages_in: u64,
    pub messages_out: u64,
    pub messages_mailbox: u64,
    pub running_time: u64,
    pub uptime: i64,
    pub state: ProcessState,
    pub parent: ErgoPid,
    pub leader: ErgoPid,
    pub log_level: LogLevel,
}

/// gen.MessageInspectApplicationTree
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ApplicationTreeMessage {
    pub node: String,
    pub application: String,
    pub processes: Vec<ProcessShortInfo>,
}