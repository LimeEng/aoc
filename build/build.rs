use std::{
    env,
    fs::File,
    io::{self, Write},
    path::Path,
};

mod benchmarks;
mod cipher;
mod discovery;
mod docs;
mod solvers;
mod tests;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PuzzleId {
    pub year: u32,
    pub day: u32,
    pub part: u32,
}

fn main() -> io::Result<()> {
    let password = env::var("AOC_KEY").unwrap();

    write("solvers.rs", &solvers::generate())?;
    write("tests.rs", &tests::generate())?;
    write("benchmarks.rs", &benchmarks::generate())?;

    docs::generate(&password)?;

    Ok(())
}

fn write<P>(filename: P, contents: &str) -> io::Result<()>
where
    P: AsRef<Path>,
{
    let out_dir = env::var("OUT_DIR").unwrap();
    let path = Path::new(&out_dir).join(filename);
    let mut output = File::create(path)?;
    writeln!(&mut output, "{contents}")
}
