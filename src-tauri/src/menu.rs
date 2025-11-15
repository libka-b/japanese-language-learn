use serde::Serialize;

#[derive(Serialize)]
pub struct Button {
    id: String,
    text: String,
}

#[tauri::command]
pub fn get_menu_buttons() -> Vec<Button> {
    vec![
        Button {
            id: "start-lesson".to_string(),
            text: "Start Lesson".to_string(),
        },
        Button {
            id: "quit".to_string(),
            text: "Quit".to_string(),
        },
    ]
}
