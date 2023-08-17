use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use crate::components::status::McStatusComponent;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! { 
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/atb-web.css"/>

        // sets the document title
        <Title text="Arun's Treasury Branches"/>
        <Link rel="icon" href="/favicon.svg"/>
        <Html attributes=AdditionalAttributes::from(vec![("data-theme", "forest")])/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
            <Navbar/>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/status" view=Status/>
                    <Route path="/idk" view=Idk/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <h1>"It worky!"</h1>
        <button class="btn btn-primary" on:click=on_click>
            "Click Me: "
            {count}
        </button>
    }
}

#[component]
fn Navbar() -> impl IntoView {

    let paths = move || { vec![
        ("Home", "/"),
        ("Server Status", "/status"),
        ("idk", "/idk"),
    ]};
    view! {
        <nav>
            <div class="flex flex-row space-x-4">
                <For
                    each=paths
                    key=|(_l, p)| *p
                    view=move |(l, p)| {
                        view! {
                            <A href=p class="btn btn-primary btn-outline">
                                {l}
                            </A>
                        }
                    }
                />

            </div>
        </nav>
    }
}


#[component]
fn Idk() -> impl IntoView {
    view! { <h1>"Idk"</h1> }
}

#[component]
fn Status() -> impl IntoView {
    view! {
        <h1>"Status"</h1>
        <McStatusComponent/>
    }
}


