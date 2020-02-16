use super::*;
use std::fs::File;
use std::io::prelude::*;

fn parse_output_file(file_name: &str) -> Vec<[[char; GRIDX]; GRIDY]> {
    let mut output_file = File::open(file_name).unwrap();
    let mut file_contents = String::new();
    output_file.read_to_string(&mut file_contents).unwrap();

    let line_itr = file_contents.split('\n');

    let mut line_count = 1;
    let mut y = 0;
    let mut output_vec: Vec<[[char; GRIDX]; GRIDY]> = Vec::new();

    let mut state: [[char; GRIDX]; GRIDY] = [[DEAD; GRIDX]; GRIDY];

    for line in line_itr {
        let mut x = 0;

        for c in line.chars() {
            if c == LIVE {
                state[y][x] = LIVE;
            }

            x += 1;
        }

        if line_count % GRIDY == 0 {
            output_vec.push(state);
            state = [[DEAD; GRIDX]; GRIDY];
            y = 0;
        } else {
            y += 1;
        }

        line_count += 1;
    }

    output_vec
}

#[test]
fn blinker_test() {
    let output_vec = parse_output_file("outputs/blinker.expected.txt");
    let mut blinker = LifeState::from_rle("inputs/blinker.rle").unwrap();

    for arr in output_vec {
        blinker.next_generation();
        for i in 0..GRIDY {
            for j in 0..GRIDX {
                if arr[i][j] != blinker.state[i][j][0] {
                    assert!(false);
                }
            }
        }
    }

    assert!(true);
}
