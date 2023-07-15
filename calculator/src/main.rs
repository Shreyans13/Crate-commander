use std::env::{args, Args};

fn operate(operator: char, n1: f32, n2: f32) -> f32 {
    match operator {
        '+' => n1 + n2,
        '-' => n1 - n2,
        '/' => n1 / n2,
        '*' | 'X' | 'x' => n1 * n2,
        _ => panic!("Operator Not found or Wrong operator"),
    }
}

fn output(n1: f32, op: char, n2: f32, n3: f32) -> String {
    format!("{} {} {} {} ", n1, op, n2, n3)
}
fn main() {
    let mut args: Args = args();
    let n1 = args.nth(1).unwrap().parse::<f32>().unwrap();
    let character = args.nth(0).unwrap().chars().next().unwrap();
    let n2 = args.nth(0).unwrap().parse::<f32>().unwrap();
    println!("{}", output(n1, character, n2, operate(character, n1, n2)));
}
