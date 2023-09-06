use crate::components::{LoginButton, Avatar};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn Navbar() -> impl IntoView {
    let paths = move || vec![("Home", "/"), ("Server Status", "/status")];
    let (logged_in, set_logged_in) = create_signal(false);

    // todo get it from login
    let uuid = "c7da90d56a054217b94a7d427cbbcad8";

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

                <Avatar uuid=uuid.into()/>
            </div>
        </nav>
        <LoginButton/>
    }
}
