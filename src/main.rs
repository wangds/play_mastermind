use std::cmp::min;

type CodePegs = [i8; 4];
type Feedback = (i8, i8);

const MIN_PEG_VALUE: i8 = 1;
const MAX_PEG_VALUE: i8 = 6;

fn format_pegs(x: &CodePegs) -> String {
    format!("{} {} {} {}", x[0], x[1], x[2], x[3])
}

fn generate_puzzle() -> CodePegs {
    [1, 2, 3, 4]
}

fn check_pegs(solution: &CodePegs, guess: &CodePegs) -> Feedback {
    let mut num_correct = 0;
    let mut num_correct_values = 0;
    let mut expected = [0; 1 + MAX_PEG_VALUE as usize];
    let mut actual = [0; 1 + MAX_PEG_VALUE as usize];

    // Find number of pegs with the correct value and position.
    for idx in 0..solution.len() {
        if solution[idx] == guess[idx] {
            num_correct += 1;
        }
        expected[solution[idx] as usize] += 1;
        actual[guess[idx] as usize] += 1;
    }

    // Find number of pegs with the correct value.
    for val in MIN_PEG_VALUE..(MAX_PEG_VALUE + 1) {
        let idx = val as usize;
        num_correct_values += min(expected[idx], actual[idx]);
    }

    (num_correct, num_correct_values - num_correct)
}

fn main() {
    let solution = generate_puzzle();
    println!("solution: {}", format_pegs(&solution));

    let guess = [1, 1, 4, 5];
    println!("guess   : {}", format_pegs(&guess));

    let (num_correct, num_incorrect_position) = check_pegs(&solution, &guess);
    println!(
        "check   : {} correct, {} incorrect position",
        num_correct, num_incorrect_position
    );
}
