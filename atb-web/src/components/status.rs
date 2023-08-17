use crate::serverfns::server_status::ping_minecraft_server;
use atb_types::McServerStatus;
use leptos::*;

#[component]
fn Status(status: Result<McServerStatus, ServerFnError>) -> impl IntoView {
    let status = status.unwrap();
    let version = format!("Version: {}", status.version);
    let players = move || status.sample.clone();

    let imgref = create_node_ref::<html::Img>();
    //https://github.com/leptos-rs/leptos/discussions/1194#discussioncomment-6196929
    create_effect(move |_| {
        use gloo::file::{Blob, ObjectUrl};

        if let Some(img) = imgref.get() {
            let favicon = status.favicon.as_slice();
            let blob = Blob::new_with_options(favicon, Some("image/png"));
            let url = ObjectUrl::from(blob);

            img.set_src(&url);
        }
    });

    // todo: use Error Boundary to handle error
    // https://leptos-rs.github.io/leptos/view/07_errors.html?highlight=error#error-handling
    view! {
        <>
            <p>{version}</p>
            <img node_ref=imgref/>
            <p>{format!("Players: {}/{}", status.online_players, status.max_players)}</p>
            <For
                each=players
                key=|(_name, id)| id.clone()
                view=move |(name, _id)| {
                    view! { <p>{name}</p> }
                }
            />
        </>
    }
}
#[component]
pub fn McStatusComponent() -> impl IntoView {
    let once = create_resource(
        // todo: add refresh button
        move || (),
        move |_| async move {
            let r = ping_minecraft_server().await;
            r
        },
    );

    let x = move || once.read().map(|v| view! { <Status status=v/> });

    view! {
        <h1>"Server Status"</h1>
        <Suspense fallback=move || view! { <p>"Loading..."</p> }>
            <div>

                {x}

            </div>
        </Suspense>
    }
}
