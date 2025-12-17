use std::{
    cmp::max,
    io::{self, BufRead},
};

fn main() {
    let lines = io::stdin().lock().lines();
    let mut v = vec![];
    let mut res = 0;
    for line in lines {
        let line = line.unwrap();
        if line == "" {
            break;
        }
        let mut val = line.split('-');
        let a = val.next().unwrap().parse::<i64>().unwrap();
        let b = val.next().unwrap().parse::<i64>().unwrap();
        v.push((a, b));
    }
    v.sort();
    let mut w = vec![];
    for (mut a, mut b) in v {
        while !w.is_empty() {
            let (c, d) = *w.last().unwrap();
            if d >= a {
                w.pop();
                a = c;
                b = max(b, d);
            } else {
                break;
            }
        }
        w.push((a, b));
    }
    for (a, b) in w {
        res += b - a + 1;
    }
    println!("{res}");
}
