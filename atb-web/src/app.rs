use {
    crate::{
        components::{navbar::Navbar, status::McStatusComponent},
        error_template::{AppError, ErrorTemplate},
        pages::*,
    },
    leptos::*,
    leptos_meta::*,
    leptos_router::*,
};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/atb-web.css"/>

        // sets the document title
        <Title text="Arun's Treasury Branches"/>
        <Link rel="icon" href="/favicon.svg"/>
        // <Html attributes=vec![("data-theme", "forest")]/>
        <Html attr:data-theme="forest"/>
        <Meta name="view-transition" content="same-origin"/>
        <Script src="https://cdn.plot.ly/plotly-2.14.0.min.js"/>

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
                    <Route path="/api/auth/callback/azure-ad" view=AuthCallback/>
                    <Route path="/signup" view=Signup/>
                    <Route path="/login" view=Login/>
                    <Route path="/transactions" view=TransactionsPage/>
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
