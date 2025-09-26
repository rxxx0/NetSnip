use crate::AppState;
use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Settings {
    pub theme: String,
    pub auto_refresh: bool,
    pub refresh_interval: u32,
    pub show_notifications: bool,
    pub self_protection: bool,
    pub gateway_protection: bool,
    pub sound_enabled: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            theme: "light".to_string(),
            auto_refresh: false,
            refresh_interval: 5,
            show_notifications: true,
            self_protection: true,
            gateway_protection: true,
            sound_enabled: false,
        }
    }
}

#[tauri::command]
pub async fn get_settings(state: State<'_, AppState>) -> Result<Settings, String> {
    // TODO: Load from database
    Ok(Settings::default())
}

#[tauri::command]
pub async fn update_settings(
    state: State<'_, AppState>,
    settings: Settings,
) -> Result<(), String> {
    let mut database = state.database.lock().await;

    // TODO: Save to database

    Ok(())
}