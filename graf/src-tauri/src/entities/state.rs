use chrono::Datelike;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use structsy::Structsy;
use tauri::{Emitter, Manager};
use chrono::NaiveDate;

#[derive(Clone)]
pub struct AppState {
    pub current_date: Arc<Mutex<NaiveDate>>,
    pub db: Arc<Mutex<Structsy>>,
}
