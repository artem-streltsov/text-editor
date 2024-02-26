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
mod filetype;
mod row;
mod terminal;
mod highlighting;

use editor::Editor;
pub use document::Document;
pub use row::Row;
pub use editor::SearchDirection;
pub use filetype::FileType;
pub use filetype::HighlightingOptions;
pub use terminal::Terminal;
pub use editor::Position;

fn main() {
    Editor::default().run();
}
