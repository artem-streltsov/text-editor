#![warn(clippy::all, clippy::pedantic)]
mod document;
mod editor;
mod row;
mod terminal;
pub use document::Document;
pub use row::Row;
use editor::Editor;
pub use terminal::Terminal;
pub use editor::Position;

fn main() {
    Editor::default().run();
}
