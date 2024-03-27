use leptos::leptos_dom::ev::SubmitEvent;
use leptos::*;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use tokio::time::{sleep, Duration};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[component]
pub fn App() -> impl IntoView {
    let (name, set_name) = create_signal(String::new());
    let (greet_msg, set_greet_msg) = create_signal(String::new());
    let (is_light_theme, set_is_light_theme) = create_signal(String::new());

    let update_name = move |ev| {
        let v = event_target_value(&ev);
        set_name.set(v);
    };

    let greet = move |ev: SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            let name = name.get_untracked();
            if name.is_empty() {
                return;
            }

            let args = to_value(&GreetArgs { name: &name }).unwrap();
            // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
            let new_msg = invoke("greet", args).await.as_string().unwrap();
            set_greet_msg.set(new_msg);
        });
    };

    let is_light_theme_fn = move || {
        spawn_local(async move {
            let is_light_theme_fetch = invoke("is_light_theme", to_value("").unwrap())
                .await
                .as_string()
                .unwrap();
            set_is_light_theme.set(is_light_theme_fetch);
            loop {
                let old_value = is_light_theme.get();
                let new_value = invoke("is_light_theme", to_value("").unwrap())
                    .await
                    .as_string()
                    .unwrap();
                if new_value != old_value {
                    set_is_light_theme.set(new_value);
                };
                // sleep(Duration::from_millis(2000)).await;
            }
        });
    };
    is_light_theme_fn();

    // let is_light_theme = create_resource(
    //     || (),
    //     |_| async move {
    //         invoke("is_light_theme", to_value("").unwrap())
    //             .await
    //             .as_string()
    //             .unwrap()
    //     },
    // );

    view! {
        <main class="container">

            <form class="row" on:submit=greet>
                <input
                    id="greet-input"
                    placeholder="Enter a name..."
                    on:input=update_name
                />
                <button type="submit">"Greet"</button>
            </form>

            <p><b>{ move || greet_msg.get() }</b></p>
            <p>{move || is_light_theme.get()}</p>
        </main>
    }
}
