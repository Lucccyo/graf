use crate::entities::state::AppState;
use chrono::Datelike;
use chrono::{Duration, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use structsy::StructsyTx;
use structsy_derive::Persistent;
use tauri::{Emitter, Manager};

pub fn timestamp(date: NaiveDate) -> i64 {
    date.and_hms_opt(0, 0, 0).unwrap().and_utc().timestamp()
}

fn timestamp_to_date(ts: i64) -> chrono::NaiveDate {
    chrono::NaiveDateTime::from_timestamp_opt(ts, 0)
        .expect("invalid timestamp")
        .date()
}

pub fn today() -> chrono::NaiveDate {
    chrono::Utc::now().date_naive()
}

fn yesterday() -> NaiveDate {
    let today = Utc::now().date_naive();
    today - Duration::days(1)
}

fn date_int() -> i32 {
    let today = today();
    today.year() * 10000 + today.month() as i32 * 100 + today.day() as i32
}

#[derive(Persistent, Serialize, Deserialize, Debug, Clone)]
pub struct Contribution {
    project_id: String,
    date: i64,
    number: i32,
}

#[derive(Serialize, Debug, Clone)]
pub struct Contribution_with_date {
    project_id: String,
    date: String,
    number: i32,
}

impl Contribution {
    pub fn new(project_id: String, date: i64, number: i32) -> Self {
        Self {
            project_id,
            date,
            number,
        }
    }
}

#[tauri::command]
pub fn get_contributions(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<Contribution_with_date>, String> {
    let db = state.db.lock().map_err(|_| "Failed to lock DB")?;
    let query = db.query::<Contribution>();
    let contributions: Vec<Contribution_with_date> = query
      .into_iter()
      .map(|(_, contribution)| Contribution_with_date {
        project_id: contribution.project_id,
        date: timestamp_to_date(contribution.date).to_string(),
        number: contribution.number,
      })
      .collect();
    Ok(contributions)
}

#[tauri::command]
pub fn create_contribution(
    state: tauri::State<'_, AppState>,
    project_id: String,
    app: tauri::AppHandle,
) -> Result<(), String> {
    let date = timestamp(today());
    let number = 0;
    let c = Contribution {
        project_id,
        date,
        number,
    };
    let db = state.db.lock().map_err(|_| "Cannot lock db")?;
    let mut tx = db.begin().map_err(|e| e.to_string())?;
    tx.insert(&c).map_err(|e| e.to_string())?;
    tx.commit().map_err(|e| e.to_string())?;
    app.get_webview_window("main")
        .ok_or("Main window not found")?
        .emit("contributions_updated", ())
        .map_err(|e| format!("Failed to emit event: {}", e))?;
    Ok(())
}

#[tauri::command]
pub fn inc_contribution(
    state: tauri::State<'_, AppState>,
    project_id: String,
    app: tauri::AppHandle,
) -> Result<(), String> {
    let date = timestamp(today());
    let db = state.db.lock().map_err(|_| "DB lock failed")?;
    let mut tx = db.begin().map_err(|e| e.to_string())?;
    let mut found = false;
    let query = db.query::<Contribution>();
    for (id, contribution) in query.into_iter() {
        if contribution.project_id == project_id && contribution.date == date {
            let updated = Contribution {
                number: contribution.number + 1,
                ..contribution
            };
            found = true;
            tx.update(&id, &updated).map_err(|e| e.to_string())?;
        }
    }
    if !found {
        let new_contribution = Contribution {
            project_id,
            date,
            number: 1,
        };
        tx.insert(&new_contribution).map_err(|e| e.to_string())?;
    }
    tx.commit().map_err(|e| e.to_string())?;
    app.get_webview_window("main")
        .ok_or("Main window not found")?
        .emit("contributions_updated", ())
        .map_err(|e| format!("Failed to emit event: {}", e))?;
    Ok(())
}

#[tauri::command]
pub fn dec_contribution(
    state: tauri::State<'_, AppState>,
    project_id: String,
    app: tauri::AppHandle,
) -> Result<(), String> {
    let date = timestamp(today());
    let db = state.db.lock().map_err(|_| "DB lock failed")?;
    let mut tx = db.begin().map_err(|e| e.to_string())?;
    let mut found = false;
    let query = db.query::<Contribution>();
    for (id, contribution) in query.into_iter() {
        if contribution.project_id == project_id && contribution.date == date {
            if contribution.number > 0 {
                let updated = Contribution {
                    number: contribution.number - 1,
                    ..contribution
                };
                tx.update(&id, &updated).map_err(|e| e.to_string())?;
            };
            found = true;
        }
    }
    if !found {
        let new_contribution = Contribution {
            project_id,
            date,
            number: 0,
        };
        tx.insert(&new_contribution).map_err(|e| e.to_string())?;
    }
    tx.commit().map_err(|e| e.to_string())?;
    app.get_webview_window("main")
        .ok_or("Main window not found")?
        .emit("contributions_updated", ())
        .map_err(|e| format!("Failed to emit event: {}", e))?;
    Ok(())
}

#[tauri::command]
pub fn delete_contribution(
    state: tauri::State<'_, AppState>,
    project_id: String,
    date: i64,
    app: tauri::AppHandle,
) -> Result<(), String> {
    let db = state.db.lock().map_err(|_| "DB lock failed")?;
    let mut tx = db.begin().map_err(|e| e.to_string())?;
    let query = db.query::<Contribution>();
    for (i, contribution) in query.into_iter() {
        if contribution.project_id == project_id && contribution.date == date {
            tx.delete(&i).map_err(|e| e.to_string())?;
            break;
        }
    }
    tx.commit().map_err(|e| e.to_string())?;
    app.get_webview_window("main")
        .ok_or("Main window not found")?
        .emit("contributions_updated", ())
        .map_err(|e| format!("Failed to emit event: {}", e))?;
    Ok(())
}
