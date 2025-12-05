use crate::emote::Emote;

use serde::{Serialize, Deserialize};

pub const SOCKET_PATH: &str = "/run/user/1000/sr71.sock";

#[derive(Debug, Serialize, Deserialize)]
pub enum KurumiControl {
    SetEmote(Emote),
    Hide,
    Show,
    NoOp,
    MoveTo
}

pub struct MoveTo {
    pub x: i16,
    pub y: i16
}


#[derive(Debug, Serialize, Deserialize)]
pub struct IPCPayload {
    pub command: KurumiControl
}
