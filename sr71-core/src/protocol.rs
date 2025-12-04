use crate::emote::Emote;

#[derive(Debug)]
pub enum KurumiControl {
    SetEmote(Emote),
    Hide,
    Show,
    MoveTo
}

pub struct MoveTo {
    pub x: i16,
    pub y: i16
}
