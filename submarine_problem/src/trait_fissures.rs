use std::str::FromStr;
use anyhow::{Result, anyhow};

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}


#[derive(Debug)]
struct Line {
    p1: Point,
    p2: Point,
}


fn get_input2() -> &'static str {
   return "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";
}

impl FromStr for Point  {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let result = s.split_once(",");
        
        if result.is_none() {
            return Err(anyhow!("Expected a point to contain a , "))
        }
        
        let (x, y) = result.unwrap();
        let x = str::parse::<i32>(x)?;
        let y = str::parse::<i32>(y)?;
        return Ok(Point {x: x, y: y });
    } 
}



impl FromStr for Line  {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
           let result = s.split_once(" -> ");
           
           if result.is_none() {
               return Err(anyhow!("Expected a point to contain a -> "))
           }
           
           let (p1, p2) = result.unwrap(); 
           let p1 = str::parse(p1)?;
           let p2 = str::parse(p2)?;
           return Ok(Line { p1, p2 });
    } 
}

impl Line {
    fn is_straight_line(&self) -> bool {
        return self.p1.x == self.p2.x || self.p1.y == self.p2.y;
    }
}


pub fn run_trait_fissures() {
    let lines = get_input2()
        .lines()
        .flat_map(|x| str::parse(x))
        .filter(|x: &Line| x.is_straight_line())
        .collect::<Vec<Line>>();

    println!("{:?}", lines);

}
