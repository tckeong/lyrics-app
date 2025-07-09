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
use controllers::{close_window, lyric_window, original_window};
use std::{
    env,
    sync::{Arc, Mutex},
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    std::thread::spawn(move || {
        let token = Arc::new(Mutex::new(Token::new()));
        let ip_addr = "127.0.0.1:8081";

        rt::System::new().block_on(async move {
            HttpServer::new(move || {
                App::new()
                    .app_data(web::Data::new(token.clone()))
                    .route("/callback", web::get().to(Token::callback))
                    .route("/token", web::post().to(Token::get_token))
            })
            .bind(ip_addr)
            .unwrap()
            .run()
            .await
            .unwrap();
        });
    });

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            login,
            login_test,
            get_username,
            close_window,
            lyric_window,
            original_window,
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
