use mongodb::bson::doc;
use native_dialog::FileDialog;
use std::env;
use std::sync::{Arc, Mutex};
mod entities;
mod structsy;
use crate::entities::project::*;
use crate::entities::contribution::*;
use crate::structsy::*;
use chrono::Utc;
use chrono::NaiveDate;
use crate::entities::state::AppState;
use tauri::Manager;


#[tauri::command]
fn load_db() -> Result<String, String> {
    match FileDialog::new()
        .set_location("~/")
        .add_filter("db file", &["db"])
        .show_open_single_file()
    {
    Ok(Some(path)) => {
        let s: String = path.display().to_string();
        Ok(s)
    }
        Ok(None) => Err("No file was selected.".to_string()),
        Err(err) => Err(format!("Error opening file dialog: {}", err)),
    }
}

fn get_today_date() -> NaiveDate {
    Utc::now().date_naive()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let app_state = AppState {
                current_date: Arc::new(Mutex::new(get_today_date())),
                db: Arc::new(Mutex::new(
                    init_db(&app.handle())
                        .expect("Error: Impossible to init the database"),
                ))
            };
            app.manage(app_state);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            load_db,
            get_projects,
            get_contributions,
            create_contribution,
            delete_contribution,
            inc_contribution,
            dec_contribution,
            create_project,
            delete_project,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
