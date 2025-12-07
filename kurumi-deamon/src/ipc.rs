use rmp_serde::{self, to_vec_named};
use sr71_core::protocol::IPCPayload;
use tokio::{io::AsyncWriteExt, net::UnixStream};

pub async fn send_payload(stream: &mut UnixStream, payload: &IPCPayload) -> anyhow::Result<()> {
    let bytes = to_vec_named(payload)?;
    stream.write_all(&bytes).await?;
    Ok(())
}
