use std::io;

fn main() {
    println!("Input the Fibonacci Term you're looking for: ");
    let mut fib_term = String::new();

    io::stdin()
        .read_line(&mut fib_term)
        .expect("Failed to read line.");

    let trimmed = fib_term.trim();

    let fib_term: i64 = match trimmed.parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid Input. Input an integer again: ");
            return;
        }
    };
    let fib_ans = calculate_fib(fib_term);
    println!("The {}th term of the Fibonacci sequence is {}", fib_term, fib_ans);
}
fn square_root_of_five() -> f64 {
    const NUM: f64 = 5.0;
    NUM.sqrt()
}
fn binet_formula_part1(fib_term: i64) -> f64 {
    let formula = (1.0 + square_root_of_five()) / 2.0;
    formula.powf(fib_term as f64)
}
fn binet_formula_part2(fib_term: i64) -> f64 {
    let formula = (1.0 - square_root_of_five()) / 2.0;
    formula.powf(fib_term as f64)
}
fn calculate_fib(fib_term: i64) -> f64 {
    let part1 = binet_formula_part1(fib_term);
    let part2 = binet_formula_part2(fib_term);
    let fibonacci = (part1 - part2) / square_root_of_five();
    fibonacci.round()
}
