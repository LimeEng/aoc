use super::{Error, read_encrypted, write_encrypted};
use crate::{
    PuzzleId,
    storage::{ENC_EXTENSION, PUZZLES_ENC_DIR},
};
use std::path::Path;

#[derive(Clone, Debug)]
pub struct Description {
    pub description: String,
    pub styles: String,
}

pub fn get_description(id: &PuzzleId, password: &str) -> Result<Description, Error> {
    let puzzle_dir = Path::new(PUZZLES_ENC_DIR)
        .join(format!("{}/{:02}/part_{}/puzzle", id.year, id.day, id.part));

    let description_path = puzzle_dir.join(format!("description.html.{ENC_EXTENSION}"));
    let styles_path = puzzle_dir.join(format!("styles.css.{ENC_EXTENSION}"));

    let description = String::from_utf8(read_encrypted(&description_path, password)?)?;
    let styles = String::from_utf8(read_encrypted(&styles_path, password)?)?;

    Ok(Description {
        description,
        styles,
    })
}

pub fn save_description(
    id: &PuzzleId,
    description: Option<&str>,
    styles: Option<&str>,
    password: &str,
) -> Result<(), Error> {
    let puzzle_dir = Path::new(PUZZLES_ENC_DIR)
        .join(format!("{}/{:02}/part_{}/puzzle", id.year, id.day, id.part));

    if let Some(html) = description {
        let path = puzzle_dir.join(format!("description.html.{ENC_EXTENSION}"));
        write_encrypted(&path, html.as_bytes(), password)?;
    }
    if let Some(css) = styles {
        let path = puzzle_dir.join(format!("styles.css.{ENC_EXTENSION}"));
        write_encrypted(&path, css.as_bytes(), password)?;
    }

    Ok(())
}
