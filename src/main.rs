mod life;

#[cfg(test)]
mod tests;

use life::*;
use std::env;
use std::io;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid arguments"));
    }

    let state = LifeState::from_rle(args[1].as_str())?;

    println!("{}", state);

    Ok(())
}
