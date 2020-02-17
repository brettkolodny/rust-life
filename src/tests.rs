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

fn states_are_equal(
    input_state: [[char; GRIDX]; GRIDY],
    output_state: [[char; GRIDX]; GRIDY],
) -> bool {
    for i in 0..GRIDY {
        for j in 0..GRIDX {
            if input_state[i][j] != output_state[i][j] {
                return false;
            }
        }
    }

    true
}

fn compare_input_output(input_file: &str, output_file: &str) -> bool {
    let output_vec = parse_output_file(output_file);
    let mut input = LifeState::from_rle(input_file).unwrap();

    for output_state in output_vec {
        let input_state = input.get_state();

        if !states_are_equal(input_state, output_state) {
            return false;
        }

        input.next_generation();
    }

    true
}

#[test]
fn blinker_test() {
    if !compare_input_output("inputs/blinker.rle", "outputs/blinker.expected.txt") {
        assert!(false);
    }

    assert!(true);
}

#[test]
fn two_blockrpent_0_test() {
    if !compare_input_output(
        "inputs/2blockrpent.rle",
        "outputs/2blockrpent-0.expected.txt",
    ) {
        assert!(false);
    }

    assert!(true);
}

#[test]
fn two_blockrpent_72_test() {
    if !compare_input_output(
        "inputs/2blockrpent.rle",
        "outputs/2blockrpent-72.expected.txt",
    ) {
        assert!(false);
    }

    assert!(true);
}

#[test]
fn pulsar_test() {
    if !compare_input_output("inputs/pulsar.rle", "outputs/pulsar.expected.txt") {
        assert!(false);
    }

    assert!(true);
}
