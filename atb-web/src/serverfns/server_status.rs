use leptos::*;

#[cfg(feature = "ssr")]
use craftping::tokio::ping;
#[cfg(feature = "ssr")]
use tokio::net::TcpStream;

use atb_types::McServerStatus;

#[server(McServerStatusInternalLeptosName, "/api", "GetJson", "ping_mc_server")]
pub async fn ping_minecraft_server() -> Result<McServerStatus, ServerFnError> {
    // todo : extract out to a variable for configurability
    let hostname = "mc.arun.gg";
    let port = 25565;
    let mut stream = TcpStream::connect((hostname, port)).await?;

    let pong: craftping::Response = ping(&mut stream, hostname, port).await?;

    let sample = match pong.sample {
        None => vec![],
        Some(s) => s
            .into_iter()
            .map(|craftping::Player { name, id }| (name, id))
            .collect::<Vec<_>>(),
    };

    let favicon = pong.favicon.unwrap_or(vec![]);

    let res = McServerStatus {
        version: pong.version,
        max_players: pong.max_players,
        online_players: pong.online_players,
        sample,
        favicon,
    };

    Ok(res)
}
