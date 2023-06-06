use tokio::sync::Mutex;

use crate::api::moon::auth::{AuthenticationError, AuthenticationResponseData};
use crate::gui::LauncherState;
use crate::storage::types::LoginSettingData;
use crate::storage::{StorageError, StorageType};

#[tauri::command]
pub async fn load_login_settings(
    state: tauri::State<'_, Mutex<LauncherState>>,
) -> Result<LoginSettingData, StorageError> {
    let mut state = state.lock().await;

    // Load data if it is not present in state cache yet
    return if let None = state.cached_login_data {
        let loaded_data = crate::storage::load_storage_data(
            StorageType::Login,
            LoginSettingData {
                uid: -1,
                remember_me: true,
            },
        )?;

        state.cached_login_data = Some(loaded_data);
        Ok(loaded_data)
    } else {
        Ok(state.cached_login_data.unwrap())
    };
}

#[tauri::command]
pub async fn login(
    state: tauri::State<'_, Mutex<LauncherState>>,
    uid: &str,
    remember_me: bool,
) -> Result<AuthenticationResponseData, AuthenticationError> {
    let mut state = state.lock().await;
    let uid_i = uid.parse::<i64>().unwrap_or(0);
    let authentication_data = crate::api::moon::auth::authenticate(&state, uid_i).await;

    // Update the session token if possible
    if let Ok(data) = &authentication_data {
        state.session_token = data.session_key.clone()
    }

    // Save the login preferences
    let login_data = LoginSettingData {
        uid: uid_i,
        remember_me,
    };
    state.cached_login_data = Some(login_data);
    match crate::storage::save_storage_data(StorageType::Login, login_data) {
        Ok(_) => {}
        // Not too critical, we can just ignore this
        _ => {}
    };

    authentication_data
}
