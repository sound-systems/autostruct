/*!
The `generator` module contains the code that that is used to generate the Rust models that map to the database
schema
*/
mod generate;
mod utils;

use std::path::Path;

use crate::database::InfoProvider;
use anyhow::{Context, Error};
use cruet::Inflector;
use tokio::{
    fs::{self, File},
    io::AsyncWriteExt,
};

mod run;
pub use run::{run, Arguments};
