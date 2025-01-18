use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::Write;

#[derive(Serialize, Deserialize, Debug)]
struct user {
    name: &'static str,
    distance: f32,
    points: i32
}

fn get_user_data() {
    let deserialized: user = serde_json::from_str(&serialized).unwrap();
}

fn write_data(user_data: &user) {
    let serialized = serde_json::to_string(&user_data).unwrap();
    fs::create_dir_all("../config");
    let mut f = File::create("../config/config.json").expect("unable to create file");
    f.write_all(serialized.as_bytes()).expect("Unable to write to file.");
    Ok(());
}

#[tauri::command]
fn test() {
    let new_user = user {
        name: "test",
        distance: 32.,
        points: 300
    };
    write_data(&new_user);
}

fn main() {
    test();
}
