use std::cmp::min;

type CodePegs = [i8; NUM_PEGS];
type Feedback = (i8, i8);

const NUM_PEGS: usize = 4;
const MIN_PEG_VALUE: i8 = 1;
const MAX_PEG_VALUE: i8 = 6;

fn format_pegs(x: &CodePegs) -> String {
    format!("{} {} {} {}", x[0], x[1], x[2], x[3])
}

fn generate_puzzle() -> CodePegs {
    [6, 6, 5, 4]
}

fn generate_all_peg_combinations() -> Vec<CodePegs> {
    // 6^4 = 1,296 combinations.
    let mut combos = Vec::new();

    for a in MIN_PEG_VALUE..(MAX_PEG_VALUE + 1) {
        for b in MIN_PEG_VALUE..(MAX_PEG_VALUE + 1) {
            for c in MIN_PEG_VALUE..(MAX_PEG_VALUE + 1) {
                for d in MIN_PEG_VALUE..(MAX_PEG_VALUE + 1) {
                    combos.push([a, b, c, d]);
                }
            }
        }
    }

    combos
}

fn pick_guess(candidates: &Vec<CodePegs>) -> CodePegs {
    let mut best_guess = candidates[0];
    let mut best_score = candidates.len() as i32;

    // Try each candidate as the guess.
    for guess in candidates {
        let mut feedback_counts = [0; (NUM_PEGS + 1) * (NUM_PEGS + 1)];

        // For each guess, determine what feedback we could get back.
        for candidate in candidates {
            let (x, y) = check_pegs(candidate, guess);
            feedback_counts[(NUM_PEGS + 1) * (x as usize) + (y as usize)] += 1;
        }

        // Minimise the worst case feedback.  A high feedback count is bad
        // because it means this guess doesn't eliminate as many candidates.
        let worst_case_score = *feedback_counts.iter().max().unwrap();
        if worst_case_score < best_score {
            best_guess = *guess;
            best_score = worst_case_score;
        }
    }

    best_guess
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

fn apply_feedback(
    candidates: Vec<CodePegs>,
    guess: &CodePegs,
    actual_feedback: &Feedback,
) -> Vec<CodePegs> {
    let mut combos = Vec::new();

    for candidate in candidates {
        let expected_feedback = check_pegs(&candidate, &guess);
        if expected_feedback == *actual_feedback {
            combos.push(candidate);
        }
    }

    combos
}

fn main() {
    let solution = generate_puzzle();
    println!("solution: {}", format_pegs(&solution));

    let mut candidates = generate_all_peg_combinations();

    for _iter in 0..6 {
        println!(" {} candidates", candidates.len());

        let guess = pick_guess(&candidates);
        println!("  guess : {}", format_pegs(&guess));

        let actual_feedback = check_pegs(&solution, &guess);
        println!(
            "  check : {} correct, {} incorrect position",
            actual_feedback.0, actual_feedback.1
        );

        candidates = apply_feedback(candidates, &guess, &actual_feedback);
        if candidates.len() == 1 {
            println!("SOLUTION: {}", format_pegs(&candidates[0]));
            break;
        }
    }
}
