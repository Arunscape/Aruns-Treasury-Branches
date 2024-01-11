use leptos::*;
use leptos_use::*;
// 1. Import our protocol bindings. This provides nothing else than the ability to serialise
// and deserialise what the server sends us.
//
// If you choose not to use these, and want to opt for manually writing javascript instead
// you MUST pay attention to the fact that some fields must be base64url safe decoded
// in the browser into Uint8Array's. There is NO VIABLE WAY to unpack json direct to a
// Uint8Array without client side JS assistance.
//
// The benefit of this wasm library is it magically does all that for you :)
use webauthn_rs_proto::*;

#[component]
pub fn SignupButton() -> impl IntoView {
    let signup = create_action(|_: &()| async {
        let window = use_window();

        // Returns `None` on the server but will not panic.
        let navigator = window.navigator();

        if let None = navigator {
            return;
        }
        let _ = navigator.unwrap().credentials().create().unwrap();

        log::error!("Logged in");

    });

    view! {
        <button class="btn btn-primary" on:click=move |_| signup.dispatch(())>
            Signup
        </button>
    }
}
