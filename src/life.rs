extern crate regex;

use regex::Regex;

use std::ffi::OsStr;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

pub const GRIDX: usize = 80;
pub const GRIDY: usize = 24;

pub const LIVE: char = 'X';
pub const DEAD: char = ' ';

pub struct LifeState {
    pub state: [[char; GRIDX]; GRIDY],
}

impl LifeState {
    pub fn from_rle(file_name: &str) -> io::Result<Self> {
        let state = {
            let file_extension = Path::new(file_name).extension();

            if let Some(ext) = file_extension {
                if ext != OsStr::new("rle") {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "Invalid file extension",
                    ));
                }

                let mut input_file = File::open(file_name)?;
                let mut file_contents = String::new();
                input_file.read_to_string(&mut file_contents)?;

                let line_itr = file_contents.split('\n');

                let mut line_number = 1;
                let mut width: Option<u32> = None;
                let mut height: Option<u32> = None;
                let mut rle_state: Vec<(u32, String)> = Vec::new();

                for line in line_itr {
                    if line_number == 1 {
                        let parameters = Regex::new(r"^x = \d+, y = \d+$").unwrap(); //match to x = int, y = int

                        if !parameters.is_match(line) {
                            return Err(io::Error::new(
                                io::ErrorKind::InvalidData,
                                "Invalid size parameters",
                            ));
                        }

                        let word_itr = line.split(' ');

                        let number_reg = Regex::new(r"\d+,?").unwrap(); //match to any number with a possible comma after it
                        for word in word_itr {
                            if number_reg.is_match(word) {
                                if width.is_none() {
                                    width = Some(word[0..(word.len() - 1)].parse().unwrap());
                                } else {
                                    height = Some(word.parse().unwrap());
                                }

                                if width == Some(0) || height == Some(0) {
                                    return Err(io::Error::new(
                                        io::ErrorKind::InvalidData,
                                        "Size parameters must be greater than 0",
                                    ));
                                }
                            }
                        }
                    } else {
                        let mut number = String::new();
                        let mut cell_type = String::new();
                        for c in line.chars() {
                            if c.is_numeric() {
                                number.push(c);
                            } else {
                                cell_type.push(c.clone());
                                if number != "" {
                                    rle_state.push((number.parse().unwrap(), cell_type));
                                } else {
                                    rle_state.push((1, cell_type));
                                }

                                number = String::new();
                                cell_type = String::new();
                            }
                        }
                    }

                    line_number += 1;
                }

                let mut initial_state: [[char; GRIDX]; GRIDY] = [[DEAD; GRIDX]; GRIDY];

                let x_start = (GRIDX / 2) - (width.unwrap() / 2) as usize;
                let y_start = (GRIDY / 2) - (height.unwrap() / 2) as usize;

                let mut current_x = x_start;
                let mut current_y = y_start;

                for (num, cell_type) in rle_state {
                    for _ in 0..num {
                        match cell_type.as_str() {
                            "b" => {
                                current_x += 1;
                                initial_state[current_y][current_x] = DEAD
                            }
                            "o" => {
                                current_x += 1;
                                initial_state[current_y][current_x] = LIVE
                            }
                            "$" => {
                                current_y += 1;
                                current_x = x_start;
                            }
                            _ => break,
                        }
                    }
                }

                initial_state
            } else {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Unable to find file extension",
                ));
            }
        };

        Ok(LifeState { state })
    }

    pub fn next_generation(&mut self) {
        println!("TODO");
    }
}

impl fmt::Display for LifeState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut state_string = String::new();

        for i in 0..GRIDY {
            for j in 0..GRIDX {
                state_string.push(self.state[i as usize][j as usize]);
            }

            state_string.push('\n');
        }

        write!(f, "{}", state_string)
    }
}
