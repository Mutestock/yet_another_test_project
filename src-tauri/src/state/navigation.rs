use std::sync::Mutex;
use tauri::State;

use crate::connection::common::{ConnectionType};

pub struct NavigationStorage {
    currently_selected_new_connection: Mutex<ConnectionType>,
}

#[tauri::command]
pub fn get_currently_selected_new_connection(
    navigation_storage: State<NavigationStorage>,
) -> Result<usize, tauri::InvokeError> {
    Ok(*navigation_storage
        .currently_selected_new_connection
        .lock()
        .unwrap() as usize)
}

#[tauri::command]
pub fn set_currently_selected_new_connection(
    connection_numeric: usize,
    navigation_storage: State<NavigationStorage>,
) -> Result<(), tauri::InvokeError> {
    Ok(*navigation_storage
        .currently_selected_new_connection
        .lock()
        .unwrap() = ConnectionType::try_from(connection_numeric).unwrap())
}

impl Default for NavigationStorage {
    fn default() -> Self {
        Self {
            currently_selected_new_connection: Mutex::new(ConnectionType::None),
        }
    }

}
