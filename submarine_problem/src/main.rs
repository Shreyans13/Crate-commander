mod fissures;
mod trait_fissures;
mod count_trees;

use crate::fissures::run;
use crate::trait_fissures::run_trait_fissures;
use crate::count_trees::run_count_trees;

// use crate::fissures::{
//     run
// };

fn get_input() -> &'static str {
    return "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2";
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn parse_line(line: &str ) -> Point {
    let (dir, amount) = line.split_once(" ").expect("must contain a empty string");
    let amount = str::parse::<i32>(amount).expect("string must be a integer");

    if dir == "forward" {
        return Point {x: amount, y: 0}
    } else if dir == "up" {
        return Point {x: 0, y: -amount }
    } else {
        return Point {x: 0, y: amount}
    }

}


// ------- problem 2
fn main() {
    let result = get_input()
        .lines()
        .map(parse_line)
        .fold(Point {x: 0,y: 0},|mut acc, point| {
            acc.x += point.x;
            acc.y += point.y;
            return acc;
    });
    println!("{:?}", result);
    println!("{:?}", result.x * result.y);
    run();
    run_trait_fissures();
    run_count_trees();
}
