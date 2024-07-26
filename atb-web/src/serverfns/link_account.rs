use leptos::*;

#[server(LinkAccountFn, "/api", "PostJson", "login")]
pub async fn link_account(
    minecraft_uuid: Uuid,
    email: String,
    jwt: String,
) -> Result<(), ServerFnError> {
    Ok(())
}
