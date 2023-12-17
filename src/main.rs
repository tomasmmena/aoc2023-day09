use std::env;
use std::fs;
use std::io::{self, BufRead};


fn get_next_in_sequence(sequence: &Vec<i64>) -> i64 {
    let diff_sequence: Vec<i64> = sequence
        .windows(2)
        .map(|w| w[1] - w[0])
        .collect();
    if diff_sequence.iter().all(|n| *n == 0) {
        *sequence.last().expect("Sequence is empty!")
    } else {
        sequence.last().expect("Sequence is empty!") + get_next_in_sequence(&diff_sequence)
    }
}

fn main() {
    let path = env::args().nth(1).expect("Missing required parameter path!");

    let data: Vec<Vec<i64>> = io::BufReader::new(
        fs::File::open(path).expect("Could not open file!"))
        .lines()
        .map(|line| {
            let text = line.expect("Could not read line!");
            text.split_whitespace().map(|n| n.parse::<i64>().expect("Invalid sequence number!")).collect()
        })
        .collect();

    let mut sum_new_values: i64 = 0;
    for sequence in data.iter() {
        sum_new_values += get_next_in_sequence(sequence);
    }

    println!("Sum of new values: {}", sum_new_values);
}

#[test]
fn test_get_next_in_sequence() {
    let sequence_1: Vec<i64> = vec![0, 3, 6, 9, 12, 15];
    assert_eq!(get_next_in_sequence(&sequence_1), 18);

    let sequence_2: Vec<i64> = vec![1, 3, 6, 10, 15, 21];
    assert_eq!(get_next_in_sequence(&sequence_2), 28);

    let sequence_3: Vec<i64> = vec![10, 13, 16, 21, 30, 45];
    assert_eq!(get_next_in_sequence(&sequence_3), 68);
}
