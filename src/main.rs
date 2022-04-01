type CodePegs = [i8; 4];

fn format_pegs(x: &CodePegs) -> String {
    format!("{} {} {} {}", x[0], x[1], x[2], x[3])
}

fn generate_puzzle() -> CodePegs {
    [1, 2, 3, 4]
}

fn main() {
    let solution = generate_puzzle();
    println!("solution: {}", format_pegs(&solution));
}
