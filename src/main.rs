mod life;

#[cfg(test)]
mod tests;

use life::*;
use std::env;
use std::io;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    println!("{:?}", args);

    let state = LifeState::from_rle(args[1].as_str())?;

    println!("{}", state);

    Ok(())
}
