use std::{
    cmp::max,
    io::{self, BufRead},
};

fn max_area(x: i64, y: i64, a: i64, b: i64) -> i64 {
    let mut res = (x - a + 1) * (y - b + 1);
    res = max(res, (x - a + 1) * (b - y + 1));
    res = max(res, (a - x + 1) * (b - y + 1));
    res = max(res, (a - x + 1) * (y - b + 1));
    return res;
}

fn main() {
    let lines = io::stdin().lock().lines();
    let mut v = vec![];
    for line in lines {
        let line = line.unwrap();
        let mut line = line.split(',');
        let x = line.next().unwrap().parse::<i64>().unwrap();
        let y = line.next().unwrap().parse::<i64>().unwrap();
        v.push((x, y));
    }
    let mut res = 0;
    for (x, y) in &v {
        for (a, b) in &v {
            res = max(res, max_area(*x, *y, *a, *b));
        }
    }
    println!("{res}");
}
