mod cipher;
mod components;

pub use cipher::SaltedKey;
pub use components::{
    Description, Puzzle, PuzzleInput, PuzzleMetadata, TestCase, decrypt_all, encrypt_all, get,
    get_all, get_description, read_input, save_description, save_input,
};

/// Directory containing plaintext puzzle files
const PUZZLES_DIR: &str = "puzzles";

/// File extension for encrypted files
const ENC_EXTENSION: &str = "enc";

/// Directory containing encrypted puzzle files
const PUZZLES_ENC_DIR: &str = "puzzles.enc";
