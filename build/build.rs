use std::{
    env,
    fs::File,
    io::{self, Write},
    path::Path,
};

mod solvers;
mod tests;

fn main() -> io::Result<()> {
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir);

    write(out_dir.join("solvers.rs"), &solvers::generate())?;
    write(out_dir.join("tests.rs"), &tests::generate())?;
    Ok(())
}

fn write<P>(target: P, contents: &str) -> io::Result<()>
where
    P: AsRef<Path>,
{
    let mut output = File::create(target)?;
    writeln!(&mut output, "{contents}")
}
