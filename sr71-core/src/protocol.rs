use crate::emote::Emote;

use serde::{Deserialize, Serialize};

pub const SOCKET_PATH: &str = "/run/user/1000/sr71.sock";

#[derive(Debug, Serialize, Deserialize)]
pub enum KurumiControl {
    SetEmote(Emote),
    Hide,
    Show,
    NoOp,
    MoveTo { x: i16, y: i16 },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IPCPayload {
    pub command: KurumiControl,
}
