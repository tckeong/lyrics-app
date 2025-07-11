// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod api;
mod controllers;
mod models;
mod utils;

use actix_web::{rt, web, App, HttpServer};
use api::server::Token;
use api::spotify_api::{self};
use controllers::authentication::{auth_check, get_username, login, login_test};
use controllers::song_details::{
    get_duration, get_id, get_image_url, get_lyrics, get_lyrics_list, get_play_status, get_time,
    save_lyrics,
};
use std::{
    env,
    sync::{Arc, Mutex},
};

#[derive(Clone, Default, Debug)]
pub struct AppState {
    token: Arc<Mutex<Option<Token>>>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app_state = AppState::default();

    tauri::Builder::default()
        .manage(app_state.clone())
        .setup(move |_| {
            std::thread::spawn(move || {
                let state = app_state.clone();
                let ip_addr = "127.0.0.1:8081";

                rt::System::new().block_on(async move {
                    HttpServer::new(move || {
                        App::new()
                            .app_data(web::Data::new(state.clone()))
                            .route("/callback", web::get().to(Token::callback))
                    })
                    .bind(ip_addr)
                    .unwrap()
                    .run()
                    .await
                    .unwrap();
                });
            });
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            login,
            login_test,
            get_username,
            get_image_url,
            get_lyrics,
            get_id,
            get_play_status,
            get_time,
            save_lyrics,
            auth_check,
            get_lyrics_list,
            get_duration
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
