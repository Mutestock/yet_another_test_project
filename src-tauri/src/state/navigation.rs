use std::sync::Mutex;
use tauri::State;

use crate::connection::common::{ConnectionType, DatabaseConnection};

pub struct NavigationStorage {
    currently_selected_new_connection: Mutex<ConnectionType>,
}

#[tauri::command]
pub fn get_currently_selected_new_connection(
    navigation_storage: State<NavigationStorage>,
) -> Result<ConnectionType, tauri::InvokeError> {
    println!(
        "{:?}",
        navigation_storage
            .currently_selected_new_connection
            .lock()
            .unwrap()
    );

    Ok(*navigation_storage
        .currently_selected_new_connection
        .lock()
        .unwrap())
}

#[tauri::command]
pub fn set_currently_selected_new_connection(
    whatevs: usize,
    navigation_storage: State<NavigationStorage>,
) -> Result<(), tauri::InvokeError> {
    Ok(*navigation_storage
        .currently_selected_new_connection
        .lock()
        .unwrap() = ConnectionType::try_from(whatevs).unwrap())
}

impl Default for NavigationStorage {
    fn default() -> Self {
        Self {
            currently_selected_new_connection: Mutex::new(ConnectionType::None),
        }
    }
}
