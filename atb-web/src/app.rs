use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
//use craftping::sync::ping;
//use std::net::TcpStream;
use craftping::tokio::ping;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! { cx,
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/atb-web.css"/>

        // sets the document title
        <Title text="Arun's Treasury Branches"/>
        <Link rel="icon" href="/favicon.svg"/>
        <Html attributes=AdditionalAttributes::from(vec![("data-theme", "forest")])/>

        // content for this welcome page
        <Router fallback=|cx| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { cx, <ErrorTemplate outside_errors/> }.into_view(cx)
        }>
            <Navbar/>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/status" view=StatusPage/>
                    <Route path="/idk" view=Idk/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(cx, 0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! { cx,
        <h1>"It worky!"</h1>
        <button class="btn btn-primary" on:click=on_click>
            "Click Me: "
            {count}
        </button>
    }
}

#[component]
fn Navbar(cx: Scope) -> impl IntoView {

    let paths = move || { vec![
        ("Home", "/"),
        ("Server Status", "/status"),
        ("idk", "/idk"),
    ]};
    view! { cx,
        <nav>
            <div class="flex flex-row space-x-4">
                <For
                    each=paths
                    key=|(l, p)| *p
                    view=move |cx, (l, p)| {
                        view! { cx,
                                <A href=p class="btn btn-primary btn-outline">{l}</A>
                        }
                    }
                />

            </div>
        </nav>
    }
}

#[server(McPing, "/api")]
pub async fn res() -> Result<serde_json::Value, ServerFnError> {
   let hostname = "mc.arun.gg";
   let port = 25565;
   use tokio::net::TcpStream;
   use tokio::io::AsyncWriteExt;
   let mut stream = TcpStream::connect((hostname, port)).await?;

   let pong = ping(&mut stream, hostname, port).await?;
   //let pong = format!("{:?}", pong);
   
   let pong = serde_json::to_value(pong)?;
   stream.shutdown().await;
   Ok(pong)
}

async fn load_data(value: i32) -> i32 {
    // fake a one-second delay
    use gloo_timers::future::TimeoutFuture;
    TimeoutFuture::new(1_000).await;
    value * 10
}

#[component]
fn StatusPage(cx: Scope) -> impl IntoView {

    //let s = create_local_resource(
    let s = create_resource(
        cx,
        ||(),
        |_| async move {res().await}
    );


    let idk = match s.read(cx) {
        Some(s) => match s {
            Ok(s) => s.to_string(),
            Err(e) => {
                e.to_string()
            },
        }
        None => "REEEEEEEEEEEEEEEEEE".to_string()
    };

    let (count, set_count) = create_signal(cx, 0);

    // create_resource takes two arguments after its scope
    let async_data = create_resource(
        cx,
        // the first is the "source signal"
        count,
        // the second is the loader
        // it takes the source signal's value as its argument
        // and does some async work
        |value| async move { load_data(value).await },
    );
    // whenever the source signal changes, the loader reloads

    // you can also create resources that only load once
    // just return the unit type () from the source signal
    // that doesn't depend on anything: we just load it once
    let stable = create_resource(cx, || (), |_| async move { load_data(1).await });

    // we can access the resource values with .read()
    // this will reactively return None before the Future has resolved
    // and update to Some(T) when it has resolved
    let async_result = move || {
        async_data
            .read(cx)
            .map(|value| format!("Server returned {value:?}"))
            // This loading state will only show before the first load
            .unwrap_or_else(|| "Loading...".into())
    };

    let loading = s.loading();
    let is_loading = move || if loading() { "Loading..." } else { "Idle." };

    let loading2 = async_data.loading();
    let is_loading2 = move || if loading2() { "Loading..." } else { "Idle." };


    


    //println!("Ping result: {:?}", pong);
    view! { cx, 
        <h1>"Status Page"</h1> 
        <div>{is_loading()}</div>
        <div>{idk}</div>
        <button
            on:click=move |_| {
                set_count.update(|n| *n += 1);
            }
        >
            "Click me"
        </button>
        <p>
            <code>"stable"</code>": " {move || stable.read(cx)}
        </p>
        <p>
            <code>"count"</code>": " {count}
        </p>
        <p>
            <code>"async_value"</code>": "
            {async_result}
            <br/>
            {is_loading2}
        </p>
    }
}

#[component]
fn Idk(cx: Scope) -> impl IntoView {
    view! { cx, <h1>"Idk"</h1> }
}
