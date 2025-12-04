use sr71_core::{protocol::KurumiControl::SetEmote, emote::Emote};

fn main() -> anyhow::Result<()> {
    println!("Kurumi daemon booting...");
    println!("Current Emote: {:?}", Emote::Idle);

    Ok(())
}
