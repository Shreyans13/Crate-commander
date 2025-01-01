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


fn parse_point(line: &str) -> Point {
    let (x,y) = line.split_once(",").expect("must be a , ");
    let x = str::parse::<i32>(x).expect("must be a int value ");
    let y = str::parse::<i32>(y).expect("must be a int value ");
    return Point {x: x, y: y };
}

fn parse_line2(line: &str) -> Line {
    let (p1, p2) = line.split_once(" -> ").expect("must be a ->");
    return Line {
        p1: parse_point(p1),
        p2: parse_point(p2),
    };
}

fn is_straight_line(line: &Line) -> bool {
    return line.p1.x == line.p2.x || line.p1.y == line.p2.y;
}



pub fn run() {
    get_input2()
        .lines()
        .map(parse_line2)
        .fold(Line{p1: Point {x: 0, y: 0}, p2: Point {x: 0, y: 0}}, |_l, li| {
            println!("{:?} == {:?}",  li, is_straight_line(&li));
            return li;
        });

}
