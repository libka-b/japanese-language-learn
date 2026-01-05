use tauri::{AppHandle, Manager};
use tauri::path::BaseDirectory;
use csv::Reader;
use serde::de::DeserializeOwned;

pub fn load_csv_entries<T: DeserializeOwned>(path: &str, handle: AppHandle) -> Vec<T> {
    let resource_path = handle
        .path()
        .resolve(path, BaseDirectory::Resource)
        .unwrap_or_else(|_| {
            panic!("Unable to resolve resource path `{}`.", path)
        });

    let mut reader = Reader::from_path(&resource_path)
        .expect(&format!("Unable to read CSV file at path: `{:?}`", resource_path));

    let records: Vec<T> = reader.deserialize().filter_map(Result::ok).collect();

    records.into_iter().collect()
}
