use crate::{emote::Emote, protocol::KurumiControl};


#[derive(Debug)]
pub struct AnimeStateMachine {
    current: Emote,
}

impl AnimeStateMachine {
    pub fn new() -> Self {
        Self {
            current: Emote::Idle,
        }
    }

    pub fn transition(&mut self, next: Emote) -> KurumiControl {
        if self.current != next {
            self.current = next;
        }

        KurumiControl::SetEmote(next)
    }
    
    pub fn current(&self) -> Emote { self.current }
}
