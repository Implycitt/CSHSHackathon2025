use serde::{Deserialize, Serialize};
use sycamore::futures::spawn_local_scoped;
use sycamore::prelude::*;
use sycamore::web::events::SubmitEvent;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[component]
pub fn App() -> View {
    let name = create_signal(String::new());
    let greet_msg = create_signal(String::new());

    let test = move |e: SubmitEvent| {
        e.prevent_default();
        spawn_local_scoped(async move {
            let args = serde_wasm_bindgen::to_value(&GreetArgs {
				name: &name.get_clone()
			})
			.unwrap();
            let new_msg = invoke("test", args).await;
            greet_msg.set(new_msg.as_string().unwrap());
        })
    };

    view! {
        main(class="header") {
            h3 {
                "test"
            }
            h3 {
                "test 2"
            }
        }
        main(class="container") {
            h1 {
                "Take your first steps towards a healthier lifestyle!"
            }

            form(class="row", on:submit=test) {
                button(r#type="submit") {
                    "Start Tracking"
                }
            }
            p {
                (greet_msg)
            }
        }
    }
}
