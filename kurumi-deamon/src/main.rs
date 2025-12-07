use rmp_serde::{from_slice, to_vec_named};
use sr71_core::{
    anime::AnimeStateMachine,
    emote::Emote,
    protocol::{IPCPayload, KurumiControl, SOCKET_PATH},
};
use std::sync::{Arc, Mutex};
use tokio::{
    fs,
    io::{AsyncReadExt, AsyncWriteExt},
    net::UnixListener,
};

mod ipc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Kurumi daemon booting...");

    if fs::metadata(SOCKET_PATH).await.is_ok() {
        fs::remove_file(SOCKET_PATH).await?;
        println!("Removed stale socket");
    }

    let listener = UnixListener::bind(SOCKET_PATH)?;
    println!(
        "Connecting... found addr: {:?}",
        listener.local_addr().unwrap()
    );

    let fsm = Arc::new(Mutex::new(AnimeStateMachine::new()));
    println!("Initial state: {:?}", fsm.lock().unwrap().current());

    loop {
        let (mut stream, _) = listener.accept().await?;
        let fsm_clone = Arc::clone(&fsm);

        tokio::spawn(async move {
            println!("Client connected!");

            // Step 1 read from client - sr71 ctl
            let mut buf = Vec::new();
            if let Err(e) = stream.read_to_end(&mut buf).await {
                eprintln!("Error: Failed to read client request: {e}");
                return;
            };

            if buf.is_empty() {
                eprintln!("Empty payload recv'd");
                return;
            };

            // Step 2 decode req
            let req: IPCPayload = match from_slice(&buf) {
                Ok(p) => p,
                Err(e) => {
                    eprintln!("Error: Failed decoding request: {e}");
                    return;
                }
            };
            println!("Daemon decoded request: {:?}", req.command);

            // step 3 build a response
            let response_cmd = {
                let mut fsm_lock = fsm_clone.lock().unwrap();
                match req.command {
                    KurumiControl::SetEmote(em) => {
                        fsm_lock.transition(em);
                        KurumiControl::SetEmote(em)
                    }
                    KurumiControl::Hide => KurumiControl::Hide,
                    KurumiControl::Show => KurumiControl::Show,
                    KurumiControl::MoveTo { x, y } => KurumiControl::MoveTo { x, y },

                    KurumiControl::NoOp => KurumiControl::NoOp,
                }
            };
            let res = IPCPayload {
                command: response_cmd,
            };
            let bytes = to_vec_named(&res).expect("Serialize failed");

            // step 4 send reply back
            if let Err(e) = stream.write_all(&bytes).await {
                eprintln!("Error: Failed to send response: {e}");
                return;
            }

            // signal done
            let _ = stream.shutdown().await;

            //tokio::time::sleep(std::time::Duration::from_millis(200)).await;
            println!("Payload delivered, closing socket");
        });
    }
}
