mod gemini;
mod lesson;
mod request;
mod response;
mod types;

pub use types::{LessonData, Translation};
pub use lesson::{generate_lesson, validate_translation};
