#![warn(clippy::all)]

mod server;
mod tools;

pub use server::MempalMcpServer;
pub use tools::{AnyJson, U8, U32, U64, USize};
