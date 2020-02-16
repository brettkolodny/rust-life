mod life;
mod parser;

use life::*;
use parser::*;
use std::io;

fn main() -> io::Result<()> {
    let state = LifeState::from_rle("inputs/2blockrpent.rle")?;

    for i in 0..GRIDY {
        for j in 0..GRIDX {
            print!("{}", state.state[i as usize][j as usize]);
        }
        println!();
    }

    Ok(())
}
