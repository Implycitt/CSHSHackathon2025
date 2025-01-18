use serde::{Deserialize, Serialize};
use sycamore::futures::spawn_local_scoped;
use sycamore::prelude::*;
use sycamore::web::events::SubmitEvent;
use wasm_bindgen::prelude::*;
use std::fs;
use std::time::Instant;
use std::fs::File;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

pub fn dist_to_points(distance: f32) -> i32 {
    let points = (distance * 160.0) as i32;
    points
}

// #[derive(Serialize, Deserialize, Debug)]
// pub struct user {
//     name: &'static str,
//     time: &Instant,
//     points: i32
// }

pub fn get_points() -> i32 {
    // let data = fs::read_to_string("../config/config.json");
    // let deserialized: user  = serde_json::from_str(&data).unwrap();
    // dist_to_points(deserialized.time)
    let points = 1200;
    points
}

// pub fn elapsed_time(start_time: Instant) -> i32 {
//     start_time.elapsed().as_secs() as i32 
// }

// pub fn start_time() {
//     let start_time = Instant::now();
//     let new_user = user {
//         name: "test",
//         time: &start_time,
//         points: 0 
//     };
//     write_data(&new_user);
// }

// pub fn write_data(user_data: &user) {
//     let serialized = serde_json::to_string(&user_data).unwrap();
//     fs::create_dir_all("../config");
//     let mut f = File::create("../config/config.json").expect("unable to create file");
//     f.write_all(serialized.as_bytes()).expect("Unable to write to file.");
// }


#[component]
pub fn App() -> View {
    let points = get_points();
    let user_time = 60;

    let start = move |e: SubmitEvent| {
        e.prevent_default();
    };
    let stop = move |e: SubmitEvent| {
        e.prevent_default();

    };

    view! {
        main(class="header") {
            // button(id="image") {
            //     // img(src="public/shop.jpg")
            // }
            h3(id="image") {
                "points: " (points)
            }
            // button(id="image") {
            //     // img(src="public/home.png")
            // }
        }
        main(class="container") {
            h1 {
                "Take your first steps towards a healthier lifestyle!"
            }

            form(class="row", on:submit=start) {
                button(r#type="submit") {
                    "Start Tracking"
                }
            form(class="row", on:submit=stop)
                button(r#type="submit") {
                    "Stop Tracking"
                }
            }
            p {
                (user_time)
            }
        }
        main(class="footer") {
            button(id="image") {
            }
            button(id="image") {
            }
            button(id="image") {
            }
        }
    }
}
