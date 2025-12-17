use std::io::{self, BufRead};

fn main() {
    let lines = io::stdin().lock().lines();
    let mut res = 0;
    let mut position = 50;
    for line in lines {
        let line = line.unwrap();
        let direction = line.chars().next().unwrap();
        let steps = line[1..].parse::<i32>().unwrap() % 100;
        if direction == 'L' {
            position = (position + 100 - steps) % 100
        } else {
            position = (position + steps) % 100;
        }
        if position == 0 {
            res += 1;
        }
    }
    println!("{res}");
}
