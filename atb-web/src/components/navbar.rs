use {
    crate::{
        components::{Avatar, SignupButton},
        serverfns::*,
    },
    leptos::*,
    leptos_meta::*,
    leptos_router::*,
};

#[component]
pub fn Navbar() -> impl IntoView {
    let paths = vec![("Home", "/"), ("Server Status", "/status")];
    let (logged_in, _set_logged_in) = create_signal(false);

    let paths = move || {
        let mut paths = paths.clone();
        if !logged_in.get() {
            paths.extend(vec![
                ("Signup", "/signup"),
                ("Login", "/login"),
                ("Transactions", "/transactions"),
            ]);
        }
        paths
    };

    // todo get it from login
    let uuid = "c7da90d56a054217b94a7d427cbbcad8";

    view! {
        <nav>
            <div class="flex flex-row space-x-4 max-h-12">
                <For each=paths key=|(_l, p)| *p let:x>
                    <A href=x.1 class="btn btn-primary btn-outline">
                        {x.0}
                    </A>
                </For>

                <Avatar uuid=uuid.into()/>
            </div>
        </nav>
    }
}
