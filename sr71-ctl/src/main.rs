use clap::Parser;
use rmp_serde::{self, from_slice, to_vec_named};
use sr71_core::{
    emote::Emote,
    protocol::{IPCPayload, KurumiControl, SOCKET_PATH},
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::UnixStream,
};

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    emote: Option<String>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let cmd = match args.emote.as_deref() {
        Some("coding") => KurumiControl::SetEmote(Emote::Coding),
        Some("sleepy") => KurumiControl::SetEmote(Emote::Sleepy),
        Some("happy") => KurumiControl::SetEmote(Emote::Happy),
        Some("gitsuccess") => KurumiControl::SetEmote(Emote::GitSuccess),
        Some("gitfail") => KurumiControl::SetEmote(Emote::GitFail),
        Some("idle") => KurumiControl::SetEmote(Emote::Idle),
        Some("annoyed") => KurumiControl::SetEmote(Emote::Annoyed),
        Some("studying") => KurumiControl::SetEmote(Emote::Studying),
        Some("chilling") => KurumiControl::SetEmote(Emote::Chilling),
        Some("shitposting") => KurumiControl::SetEmote(Emote::Shitposting),
        _ => KurumiControl::NoOp,
    };

    let payload = IPCPayload { command: cmd };

    println!("Connecting to SOCKET: {}", SOCKET_PATH);
    let mut stream = UnixStream::connect(SOCKET_PATH).await?;

    let bytes = to_vec_named(&payload)?;
    stream.write_all(&bytes).await?;
    stream.shutdown().await?;

    let mut buf = Vec::new();
    stream.read_to_end(&mut buf).await?;
    if !buf.is_empty() {
        let res: IPCPayload = from_slice(&buf)?;
        println!("Recv'd from daemon: {:?}", res);
    }

    Ok(())
}
