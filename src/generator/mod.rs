/*!
The `generator` module contains the code that that is used to generate the Rust models that map to the database
schema
*/
mod code;
mod generate;
mod runner;
mod utils;
pub use runner::{run, Arguments};
