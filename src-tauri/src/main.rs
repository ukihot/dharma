use crate::services::greet;
use crate::services::line_out;
use crate::services::raid_out;
use crate::services::raid_success;

mod common;
mod infra;
mod models;
mod services;

use tauri::{Manager, Window};

// the payload type must implement `Serialize` and `Clone`.
#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

// init a background process on the command, and emit periodic events only to the window that used the command
#[tauri::command]
fn init_process(window: Window) {
    std::thread::spawn(move || loop {
        window
            .emit(
                "event-name",
                Payload {
                    message: "Tauri is awesome!".into(),
                },
            )
            .unwrap();
    });
}
fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // `main` here is the window label; it is defined on the window creation or under `tauri.conf.json`
            // the default value is `main`. note that it must be unique
            let main_window = app.get_window("main").unwrap();
            let scorer_window = app.get_window("scorer").unwrap();
            // listen to the `event-name` (emitted on the `main` window)
            let id = main_window.listen("event-name", |event| {
                println!("got window event-name with payload {:?}", event.payload());
            });
            // unlisten to the event using the `id` returned on the `listen` function
            // an `once` API is also exposed on the `Window` struct
            scorer_window.unlisten(id);

            // emit the `event-name` event to the `main` window
            main_window
                .emit(
                    "event-name",
                    Payload {
                        message: "Tauri is awesome!".into(),
                    },
                )
                .unwrap();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![init_process])
        .run(tauri::generate_context!())
        .expect("failed to run app");
}
