use std::io::{self, BufRead};

fn main() {
    let lines = io::stdin().lock().lines();
    let mut part1 = true;
    let mut v = vec![];
    let mut res = 0;
    for line in lines {
        let line = line.unwrap();
        if line == "" {
            part1 = false;
            continue;
        }
        if part1 {
            let mut val = line.split('-');
            let a = val.next().unwrap().parse::<i64>().unwrap();
            let b = val.next().unwrap().parse::<i64>().unwrap();
            v.push((a, b));
        } else {
            let x = line.parse::<i64>().unwrap();
            for (a, b) in &v {
                if x >= *a && x <= *b {
                    res += 1;
                    break;
                }
            }
        }
    }
    println!("{res}");
}
