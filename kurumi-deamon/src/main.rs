use sr71_core::{anime::AnimeStateMachine, emote::Emote};

fn main() -> anyhow::Result<()> {
    println!("Kurumi daemon booting...");

    let mut fsm = AnimeStateMachine::new();
    println!("Current Emote: {:?}", fsm.current());

    fsm.transition(Emote::Coding);
    println!("Transitioned to -> Emote: {:?}", fsm.current());

    Ok(())
}
