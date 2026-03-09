use structsy::{Structsy, StructsyError};
use tauri::{AppHandle, Manager};
use crate::entities::project::Project;
use crate::entities::contribution::Contribution;

pub fn init_db(app: &AppHandle) -> Result<Structsy, StructsyError> {
    let path = app
        .path()
        .app_data_dir()
        .unwrap()
        .join("db.db");

    let db = Structsy::open(path)?;
    let _ = db.define::<Project>();
    let _ = db.define::<Contribution>();

    Ok(db)
}