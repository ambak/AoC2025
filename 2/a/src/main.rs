use std::io::{self, BufRead};

fn dec_len(mut x: i64) -> i64 {
    let mut res = 0;
    while x > 0 {
        res += 1;
        x /= 10;
    }
    res
}

fn smallest_dec_len(x: i64) -> i64 {
    let mut res = 1;
    for _ in 0..(x - 1) {
        res *= 10;
    }
    res
}

fn is_repeated(x: i64) -> bool {
    let x_len = dec_len(x);
    if x_len % 2 == 1 {
        return false;
    }
    let mut y = x;
    for _ in 0..(x_len / 2) {
        y /= 10;
    }
    let mut z = y;
    for _ in 0..(x_len / 2) {
        z *= 10;
    }
    z += y;

    x == z
}

fn next_repeated(mut x: i64) -> i64 {
    let mut x_len = dec_len(x);
    if x_len % 2 == 1 {
        x = smallest_dec_len(x_len + 1);
        x_len = dec_len(x);
    }
    let mut modulo = 1i64;
    for _ in 0..(x_len / 2) {
        modulo *= 10;
    }
    let a = x % modulo;
    let mut y = x;
    for _ in 0..(x_len / 2) {
        y /= 10;
    }
    if a >= y {
        let y_len = dec_len(y);
        y += 1;
        if y_len < dec_len(y) {
            x_len += 2;
        }
    }
    let mut z = y;
    for _ in 0..(x_len / 2) {
        z *= 10;
    }
    z += y;
    z
}

fn main() {
    let mut lines = io::stdin().lock().lines();
    let line = lines.next().unwrap().unwrap();
    let mut res = 0i64;
    for val in line.split(',') {
        let mut v = val.split('-');
        let mut a = v.next().unwrap().parse::<i64>().unwrap();
        let b = v.next().unwrap().parse::<i64>().unwrap();
        let a_len = dec_len(a);
        if a_len % 2 == 1 {
            a = smallest_dec_len(a_len + 1);
        }
        if is_repeated(a) && a <= b {
            res += a;
        }
        loop {
            a = next_repeated(a);
            if a <= b {
                res += a;
            } else {
                break;
            }
        }
    }
    println!("{res}");
}
