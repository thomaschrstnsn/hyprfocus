//! Minimal client query against Hyprland's IPC socket.
//!
//! The `hyprland` crate's `Client` struct requires fields (e.g. a numeric
//! workspace `id`) that Hyprland >= 0.57 no longer emits, so deserialization
//! fails. We only need a handful of fields, so query `j/clients` ourselves and
//! deserialize into a lenient local struct.

use std::{
    env,
    io::{Read as _, Write as _},
    os::unix::net::UnixStream,
    path::PathBuf,
};

use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("HYPRLAND_INSTANCE_SIGNATURE is not set; is Hyprland running?")]
    NoInstance,

    #[error("socket io error")]
    Io(#[from] std::io::Error),

    #[error("failed to parse clients json")]
    Json(#[from] serde_json::Error),
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Client {
    pub address: String,
    #[serde(default)]
    pub class: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub initial_class: String,
    #[serde(default)]
    pub initial_title: String,
    #[serde(rename = "focusHistoryID", default)]
    pub focus_history_id: i64,
}

fn socket_path() -> Result<PathBuf, Error> {
    let signature = env::var("HYPRLAND_INSTANCE_SIGNATURE").map_err(|_| Error::NoInstance)?;
    let runtime_dir = env::var("XDG_RUNTIME_DIR").unwrap_or_else(|_| "/tmp".into());

    let candidates = [
        PathBuf::from(&runtime_dir).join("hypr").join(&signature),
        PathBuf::from("/tmp/hypr").join(&signature),
    ];
    let dir = candidates
        .iter()
        .find(|p| p.exists())
        .unwrap_or(&candidates[0]);
    Ok(dir.join(".socket.sock"))
}

pub fn get() -> Result<Vec<Client>, Error> {
    let mut stream = UnixStream::connect(socket_path()?)?;
    stream.write_all(b"j/clients")?;
    let mut response = String::new();
    stream.read_to_string(&mut response)?;
    Ok(serde_json::from_str(&response)?)
}
