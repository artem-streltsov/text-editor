#![warn(clippy::all, clippy::pedantic, clippy::restriction)]
#![allow(
    clippy::missing_docs_in_private_items,
    clippy::implicit_return,
    clippy::shadow_reuse,
    clippy::print_stdout,
    clippy::wildcard_enum_match_arm,
    clippy::else_if_without_else
)]
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
