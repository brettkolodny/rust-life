mod life;
mod term;

#[cfg(test)]
mod tests;

use life::*;
use std::env;
use std::io;
use std::{thread, time};
use term::*;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let mut state = LifeState::from_rle(args[1].as_str())?;

        if args.len() == 2 {
            loop {
                clear_term();
                println!("{}", state);
                thread::sleep(time::Duration::from_millis(100));
                state.next_generation();
            }
        } else if args.len() > 3 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Invalid arguments",
            ));
        }

        let num_generations = {
            let generation_parse = args[2].parse::<u32>();

            if let Ok(v) = generation_parse {
                v
            } else {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Generation number must be a valid integer",
                ));
            }
        };

        for _ in 0..num_generations {
            state.next_generation();
        }

        println!("{}", state);
    }

    Ok(())
}
