use eframe::epaint::Color32;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::sync::mpsc::{Receiver, Sender};

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum ControlMessage {
    Play(u64),
    Pause(u64),
    ChangeStem(u64, usize),
    SyncPlaybackStatus,
    Seek(u64, f64),
    Loop(u64, bool),
    Mute(u64, bool),
    SetVolume(u64, f64),
    Delete(u64),
}

#[derive(PartialEq, Debug, Clone)]
pub enum ImportMessage {
    Cancelled,
    Update(u64, ItemImportStatus),
    Finished(Vec<Item>),
}

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum ItemImportStatus {
    Queued(String),
    InProgress,
    Finished,
    Failed(String),
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Serialize, Deserialize)]
pub struct Stem {
    pub tag: String,
    pub path: String,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Serialize, Deserialize)]
pub enum ItemStatus {
    Stopped,
    Loading,
    Playing,
    Paused,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub id: u64,
    pub name: String,
    pub stems: Vec<Stem>,
    pub current_stem: usize,
    pub volume: f64,
    pub muted: bool,
    pub looped: bool,
    pub status: ItemStatus,
    pub colour: Color32,
    pub bars: Vec<f32>,
    /// The position within the track, in seconds.
    ///
    /// This should only ever be read, since it is animated by target_position.
    pub position: f64,
    /// The target (real) position within the track, in seconds.
    ///
    /// This is effectively owned by the playback thread.
    /// Changes from elsewhere will be overwritten.
    pub target_position: f64,
    pub duration: f64,
}

#[derive(PartialEq, Debug, Clone, Default, Serialize, Deserialize)]
pub struct Model {
    pub search_query: String,
    pub items: Vec<Item>,
    pub id_counter: u64,
}

// TODO convert to a struct
pub type ImportStatus = (Vec<(u64, String, ItemImportStatus)>, Vec<Item>);
pub type SharedImportStatus = Arc<RwLock<ImportStatus>>;

pub struct SharedModel {
    pub import_state: Option<(Receiver<ImportMessage>, SharedImportStatus)>,
    pub play_channel: Sender<ControlMessage>,
    pub model: Arc<RwLock<Model>>,
}
