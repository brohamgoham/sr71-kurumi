
use sr71_core::{
    anime::AnimeStateMachine,
    emote::Emote,
    protocol::{SOCKET_PATH, KurumiControl, IPCPayload},
};
use tokio::net::UnixListener;

mod ipc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Kurumi daemon booting...");

    let listener = UnixListener::bind(SOCKET_PATH)?;

    let mut fsm = AnimeStateMachine::new();
    println!("Initial state: {:?}", fsm.current());
    
    loop {
        let (mut stream, _) = listener.accept().await?;

        tokio::spawn(async move {
            let payload = IPCPayload {
                command: KurumiControl::SetEmote(Emote::Coding)
            };

            println!("client connected -> sening payload");
            ipc::send_payload(&mut stream, &payload).await.unwrap();
        });
    }

}
