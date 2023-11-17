use crate::serverfns::server_status::ping_minecraft_server;
use atb_types::McServerStatus;
use leptos::*;

#[component]
fn Status(status: Result<McServerStatus, ServerFnError>) -> impl IntoView {
    let imgref = create_node_ref::<html::Img>();
    //https://github.com/leptos-rs/leptos/discussions/1194#discussioncomment-6196929
    let s = status.clone();
    create_effect(move |_| {
        use gloo::file::{Blob, ObjectUrl};

        if let (Some(img), Ok(status)) = (imgref.get(), &s) {
            let favicon = status.favicon.as_slice();
            let blob = Blob::new_with_options(favicon, Some("image/png"));
            let url = ObjectUrl::from(blob);

            img.set_src(&url);
        }
    });

    view! {
        <ErrorBoundary fallback=|errs| {
            view! {
                <p>"Error"</p>
                <ul>
                    {move || {
                        errs()
                            .into_iter()
                            .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                            .collect_view()
                    }}

                </ul>
            }
        }>

            {
                let status = status.clone().unwrap();
                view! {
                    <p>
                        Version:
                        {status.version}
                    </p>
                    <img node_ref=imgref/>
                    <p>{format!("Players: {}/{}", status.online_players, status.max_players)}</p>
                    <For each=move || status.sample.clone() key=|(_name, id)| id.clone() let:player>

                        <p>{player.0}</p>

                    </For>
                }
            }

        </ErrorBoundary>
    }
}
#[component]
pub fn McStatusComponent() -> impl IntoView {
    let once = create_resource(
        // todo: add refresh button
        || (),
        |_| async move { ping_minecraft_server().await },
    );

    //let x = move || once.read().map(|v| view! { <Status status=v/> });
    //

    view! {
        <h1>"Server Status"</h1>
        <Suspense fallback=move || view! { <p>"Loading..."</p> }>
            <div>

                {once.read().map(|v| view! { <Status status=v/> })}

            </div>
        </Suspense>
    }
}
