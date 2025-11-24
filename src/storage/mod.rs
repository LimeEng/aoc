mod cipher;
mod operations;

pub use cipher::SaltedKey;
pub use operations::{
    Error, Puzzle, PuzzleMetadata, TestCase, decrypt_all, encrypt_all, get, get_all, list,
};

/// Directory containing plaintext puzzle files
const PUZZLES_DIR: &str = "puzzles";

/// File extension for encrypted files
const ENC_EXTENSION: &str = "enc";

/// Directory containing encrypted puzzle files
const PUZZLES_ENC_DIR: &str = "puzzles.enc";
