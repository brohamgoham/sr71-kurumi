use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Eq, Copy, Serialize, Deserialize)]
pub enum Emote {
    Idle,
    Sleepy,
    Happy,
    Coding,
    GitSuccess,
    GitFail,
    Annoyed,
    Studying,
    Shitposting,
    Chilling,
}
