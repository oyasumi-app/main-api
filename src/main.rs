mod api;
mod core;
mod entity;
mod migration;
mod security;

mod v1;

pub use crate::core::identifiers::*;
pub use api::AppState;

fn main() {
    api::main();
}
