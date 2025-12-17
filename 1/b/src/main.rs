use std::io::{self, BufRead};

fn main() {
    let lines = io::stdin().lock().lines();
    let mut res = 0;
    let mut position = 50;
    for line in lines {
        let line = line.unwrap();
        let direction = line.chars().next().unwrap();
        let mut steps = line[1..].parse::<i32>().unwrap();
        res += steps / 100;
        steps %= 100;
        if direction == 'L' {
            position -= steps;
            if position <= 0 {
                if position + steps != 0 {
                    res += 1;
                }
                position = (position + 100) % 100;
            }
        } else {
            position += steps;
            if position >= 100 {
                res += 1;
                position %= 100;
            }
        }
    }
    println!("{res}");
}
