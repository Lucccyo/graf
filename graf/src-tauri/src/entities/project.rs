use crate::entities::state::AppState;
use serde::{Deserialize, Serialize};
use structsy::{Filter, StructsyTx};
use structsy_derive::{queries, Persistent};
use tauri::{Emitter, Manager};
use crate::entities::contribution::*;
use uuid::Uuid;


#[derive(Persistent, Serialize, Deserialize, Debug, Clone)]
pub struct Project {
  id: String,
  color: String,
  title: String,
  reason: String,
}

#[queries(Project)]
trait ProjectQuery {
    fn by_title(self, title: String) -> Self;
}

#[tauri::command]
pub fn get_projects(state: tauri::State<'_, AppState>)
  -> Result<Vec<Project>, String>
{
  let db = state.db.lock().map_err(|_| "Failed to lock DB")?;
  let query = db.query::<Project>();
  let projects: Vec<Project> = query
    .into_iter()
    .map(|(_, project)| project)
    .collect();
  Ok(projects)
}

#[tauri::command]
pub fn create_project(
  state: tauri::State<'_, AppState>,
  color: String, title: String, reason: String,
  app: tauri::AppHandle
) -> Result<(), String> {
  let date = timestamp(today());
  let id = Uuid::new_v4().to_string();
  let p = Project {
    id: id.clone(),
    color,
    title,
    reason
  };
  let set_contribution = Contribution::new(id.clone(), date, 0);
  let sclone = state.clone();
  let db = sclone.db.lock().map_err(|_| "Cannot lock db")?;
  let mut tx = db.begin().map_err(|e| e.to_string())?;
  tx.insert(&p).map_err(|e| e.to_string())?;
  tx.insert(&set_contribution).map_err(|e| e.to_string())?;
  tx.commit().map_err(|e| e.to_string())?;
  app.get_webview_window("main")
    .ok_or("Main window not found")?
    .emit("projects_updated", ())
    .map_err(|e| format!("Failed to emit event: {}", e))?;
  app.get_webview_window("main")
    .ok_or("Main window not found")?
    .emit("contributions_updated", ())
    .map_err(|e| format!("Failed to emit event: {}", e))?;
  Ok(())
}

#[tauri::command]
pub fn delete_project(
  state: tauri::State<'_, AppState>, id: String,
  app: tauri::AppHandle)
  -> Result<(), String>
{
  let db = state.db.lock().map_err(|_| "DB lock failed")?;
  let mut tx = db.begin().map_err(|e| e.to_string())?;
  let query = db.query::<Project>();
  for (i, project) in query.into_iter() {
    if project.id == id {
      tx.delete(&i).map_err(|e| e.to_string())?;
      break;
    }
  }
  tx.commit().map_err(|e| e.to_string())?;
  app.get_webview_window("main")
    .ok_or("Main window not found")?
    .emit("projects_updated", ())
    .map_err(|e| format!("Failed to emit event: {}", e))?;
  Ok(())
}