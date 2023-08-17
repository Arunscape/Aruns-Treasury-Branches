use leptos::*;
use serde_json::Value;

#[cfg(feature="ssr")]
use tokio::net::TcpStream;
#[cfg(feature="ssr")]
use craftping::tokio::ping;
use serde::{Serialize, Deserialize};

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct McStatus {
    pub version: String,
    pub max_players: usize,
    pub online_players: usize,
    pub sample: Vec<(String, String)>,
    pub favicon: Vec<u8>,
}

#[server(McPing, "/api")]
pub async fn ping_minecraft_server() -> Result<McStatus, ServerFnError> {
    // todo : extract out to a variable for configurability
   let hostname = "mc.arun.gg";
   let port = 25565;
   let mut stream = TcpStream::connect((hostname, port)).await?;

   let pong: craftping::Response = ping(&mut stream, hostname, port).await?;

   let sample = match pong.sample {
       None => vec![],
       Some(s) => s.into_iter().map(|craftping::Player { name, id }| (name, id)).collect::<Vec<_>>()
   };

   let favicon = pong.favicon.unwrap_or(vec![]);

   let res = McStatus {
       version: pong.version,
       max_players: pong.max_players,
       online_players: pong.online_players,
       sample,
       favicon,
   };
   
   Ok(res)
}

#[component]
fn Status(cx: Scope, status: McStatus) -> impl IntoView {
    let version = format!("Version: {}", status.version);
    let players = move || status.sample.clone();

    let imgref = create_node_ref::<html::Img>(cx);
    //https://github.com/leptos-rs/leptos/discussions/1194#discussioncomment-6196929
    create_effect(cx, move |_| {
        use gloo::file::{Blob, ObjectUrl};

        if let Some(img) = imgref.get() {

            let favicon = status.favicon.as_slice();
            let blob = Blob::new_with_options(favicon, Some("image/png"));
            let url = ObjectUrl::from(blob);

            img.set_src(&url);

        }
        
    });

    view! { cx,
        <p>{version}</p>
        // <img src=url/>
        <img node_ref=imgref/>
        <p>{format!("Players: {}/{}", status.online_players, status.max_players)}</p>
        <For
            each=players
            key=|(_name, id)| id.clone()
            view=move |cx, (name, _id)| {
                view! { cx, <p>{name}</p> }
            }
        />
    }
}
#[component]
pub fn McStatusComponent(cx: Scope) -> impl IntoView {


    let once = create_resource(cx, move || () , move |_| async move  { 
        let r = ping_minecraft_server().await;
        let r = r.unwrap_or(McStatus::default());
        r
    });


    //let x = move || { once.read(cx).map(|v| view! { cx, <p>{format!("{:?}", v)}</p> }) };
    let x = move || { once.read(cx).map(|v| view! { cx, <Status status=v/> }) };

    view! { cx,
        <h1>"Server Status"</h1>
        <Suspense fallback=move || view! { cx, <p>"Loading..."</p> }>
            <div>

                {x}
            // {move || { once.read(cx).map(|v| view! { cx, <p>{v.to_string()}</p> }) }}

            </div>
        </Suspense>
    }

}

