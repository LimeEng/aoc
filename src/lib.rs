#![allow(clippy::missing_panics_doc)]
#![allow(clippy::missing_errors_doc)]
// TODO: Write better code
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_sign_loss)]

pub mod cipher;
pub mod input_storage;
pub mod years;

include!(concat!(env!("OUT_DIR"), "/solvers.rs"));
