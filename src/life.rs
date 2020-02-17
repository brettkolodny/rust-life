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
    state: [[[char; 2]; GRIDX]; GRIDY],
    current_layer: usize,
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

                let mut first_line_parsed = false;
                let mut width: Option<u32> = None;
                let mut height: Option<u32> = None;
                let mut rle_state: Vec<(u32, String)> = Vec::new();

                for line in line_itr {
                    if line.starts_with('#') {
                        continue;
                    }

                    if !first_line_parsed {
                        let parameters = Regex::new(r"^x = \d+, y = \d+").unwrap(); //match to x = int, y = int

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
                                } else if height.is_none() {
                                    let alt_y_reg = Regex::new(r"\d+,").unwrap();

                                    if alt_y_reg.is_match(word) {
                                        height = Some(word[0..(word.len() - 1)].parse().unwrap());
                                    } else {
                                        height = Some(word.parse().unwrap());
                                    }
                                }

                                if width == Some(0) || height == Some(0) {
                                    return Err(io::Error::new(
                                        io::ErrorKind::InvalidData,
                                        "Size parameters must be greater than 0",
                                    ));
                                }
                            }
                        }

                        first_line_parsed = true;
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
                }

                let mut initial_state: [[[char; 2]; GRIDX]; GRIDY] = [[[DEAD; 2]; GRIDX]; GRIDY];

                let x_start = (GRIDX - (width.unwrap() as usize)) / 2;
                let y_start = (GRIDY - (height.unwrap() as usize)) / 2;

                let mut current_x = x_start;
                let mut current_y = y_start;

                for (num, cell_type) in rle_state {
                    for _ in 0..num {
                        match cell_type.as_str() {
                            "b" => {
                                initial_state[current_y][current_x][0] = DEAD;
                                current_x += 1
                            }
                            "o" => {
                                initial_state[current_y][current_x][0] = LIVE;
                                current_x += 1
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

        Ok(LifeState {
            state,
            current_layer: 0,
        })
    }

    #[allow(dead_code)]
    pub fn get_state(&self) -> [[char; GRIDX]; GRIDY] {
        let mut state = [[DEAD; GRIDX]; GRIDY];

        for i in 0..GRIDY {
            for j in 0..GRIDX {
                state[i][j] = self.state[i][j][self.current_layer];
            }
        }

        state
    }

    pub fn next_generation(&mut self) {
        let next_layer = {
            if self.current_layer == 0 {
                1
            } else {
                0
            }
        };

        for i in 0..GRIDY {
            for j in 0..GRIDX {
                if self.cell_is_alive(j as isize, i as isize) {
                    self.state[i][j][next_layer] = LIVE;
                } else {
                    self.state[i][j][next_layer] = DEAD;
                }
            }
        }

        self.current_layer = next_layer;
    }

    fn cell_is_alive(&self, x: isize, y: isize) -> bool {
        let mut num_alive = 0;

        let x_start: isize = x - 1;
        let y_start: isize = y - 1;

        for i in y_start..(y + 2) {
            if i < 0 || i > (GRIDY as isize) - 1 {
                continue;
            }

            for j in x_start..(x + 2) {
                if j < 0 || j > (GRIDX as isize) - 1 {
                    continue;
                }

                if j == x && i == y {
                    continue;
                }

                if self.state[i as usize][j as usize][self.current_layer] == LIVE {
                    num_alive += 1;
                }
            }
        }

        let current_cell = self.state[y as usize][x as usize][self.current_layer];

        if (num_alive == 2 && current_cell == LIVE) || num_alive == 3 {
            return true;
        }

        false
    }
}

impl fmt::Display for LifeState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut state_string = String::new();

        for i in 0..GRIDY {
            for j in 0..GRIDX {
                state_string.push(self.state[i as usize][j as usize][self.current_layer]);
            }

            state_string.push('\n');
        }

        write!(f, "{}", state_string)
    }
}
