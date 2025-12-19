use crate::cipher::decrypt_file;
use crate::{PuzzleId, discovery::discover_docs, write};
use std::{io, path::PathBuf};

pub fn generate(password: &str) -> io::Result<()> {
    println!("cargo:rerun-if-changed=puzzles.enc");
    let docs = discover_docs();
    generate_docs(password, &docs)
}

fn generate_docs(password: &str, docs: &[PuzzleId]) -> io::Result<()> {
    for id in docs {
        let path = to_path(id);
        if let Ok(html) = decrypt_file(&path, password) {
            let filename = format!("aoc_{}_{:02}_{}.html", id.year, id.day, id.part);
            write(&filename, &html)?;
        }
    }
    Ok(())
}

fn to_path(id: &PuzzleId) -> PathBuf {
    format!(
        "puzzles.enc/{}/{:02}/part_{}/puzzle/description.html.enc",
        id.year, id.day, id.part
    )
    .into()
}
