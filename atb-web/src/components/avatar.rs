use leptos::*;


#[component]
pub fn Avatar(uuid: String) -> impl IntoView {
    let src = format!("https://crafatar.com/avatars/{}", uuid);
    view! {
        <img src=src class="avatar"/>
    }
}
