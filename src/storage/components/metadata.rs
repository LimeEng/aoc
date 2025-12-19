use super::{Error, read_encrypted};
use crate::{
    PuzzleId,
    storage::{ENC_EXTENSION, PUZZLES_ENC_DIR},
};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PuzzleMetadata {
    pub title: String,
}

pub fn read_metadata(id: &PuzzleId, password: &str) -> Result<PuzzleMetadata, Error> {
    let meta_path = Path::new(PUZZLES_ENC_DIR)
        .join(format!("{}/{:02}", id.year, id.day))
        .join(format!("meta.toml.{ENC_EXTENSION}"));

    let bytes = read_encrypted(&meta_path, password)?;
    let metadata: PuzzleMetadata =
        toml::from_str(&String::from_utf8_lossy(&bytes)).map_err(|_| Error::Parse)?;
    Ok(metadata)
}
