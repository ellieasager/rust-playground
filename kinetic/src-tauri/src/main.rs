// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::Serialize;
use tauri::Manager;
use tauri::State;


#[derive(Serialize)]
struct RequestBody {
    name: String,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn greet<'r>(
    state: State<'r, ReqwestClient>,
    name: String,
)-> Result<String, ()> {
    let reqwest_client = &state.inner().0;
    let url = "http://localhost:8080/greet";

    let body = RequestBody {
        name,
    };

    let response = reqwest_client
        .post(url)
        .json(&body)
        .send()
        .await
        .map_err(|_| ())?;

    println!("response: {:#?}", response);

    // // Extract the response body
    let body = response.text().await.map_err(|_| ())?;
    println!("body: {}", body);

    Ok(body)
}

#[tauri::command]
async fn check<'r>(state: State<'r, ReqwestClient>) -> Result<String, ()> {
    println!("in check");
    let reqwest_client = &state.inner().0;
    let url = "http://localhost:8080/check";
    let response = reqwest_client.get(url).send().await;
    let body = response
        .expect("REASON")
        .text()
        .await
        .map_err(|e| e.to_string());
    let result = body.map_err(|e| e.to_string()).unwrap();
    Ok(result)
}

struct ReqwestClient(reqwest::Client);

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            app.manage(ReqwestClient(reqwest::Client::new()));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![check, greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
