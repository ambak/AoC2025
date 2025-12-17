use std::{
    collections::HashSet,
    io::{self, BufRead},
};

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

fn is_repeated(x: i64, step: i64) -> bool {
    let x_len = dec_len(x);
    if x_len % step != 0 {
        return false;
    }
    let mut y = x;
    for _ in 0..(x_len / step) * (step - 1) {
        y /= 10;
    }
    let mut z = y;
    for _ in 0..(step - 1) {
        for _ in 0..(x_len / step) {
            z *= 10;
        }
        z += y;
    }

    x == z
}

fn next_repeated(mut x: i64, step: i64) -> i64 {
    let mut x_len = dec_len(x);
    if x_len % step != 0 {
        x = smallest_dec_len(x_len + 1);
        x_len = dec_len(x);
    }
    let mut y = x;
    for _ in 0..(x_len / step) * (step - 1) {
        y /= 10;
    }
    let mut z = y;
    for _ in 0..(step - 1) {
        for _ in 0..(x_len / step) {
            z *= 10;
        }
        z += y;
    }
    if x >= z {
        let y_len = dec_len(y);
        y += 1;
        if dec_len(y) > y_len {
            x_len += step;
        }
        z = y;
        for _ in 0..(step - 1) {
            for _ in 0..(x_len / step) {
                z *= 10;
            }
            z += y;
        }
    }
    z
}

fn main() {
    let mut lines = io::stdin().lock().lines();
    let line = lines.next().unwrap().unwrap();
    let mut res = 0i64;
    for val in line.split(',') {
        let mut v = val.split('-');
        let mut a = v.next().unwrap().parse::<i64>().unwrap();
        let aa = a;
        let b = v.next().unwrap().parse::<i64>().unwrap();
        let a_len = dec_len(a);
        let mut m: HashSet<i64> = HashSet::new();
        for step in 2..20 {
            a = aa;
            if a_len % step != 0 {
                a = smallest_dec_len(a_len + 1);
            }
            if is_repeated(a, step) && a <= b && !m.contains(&a) {
                res += a;
                m.insert(a);
            }
            loop {
                a = next_repeated(a, step);
                if a <= b {
                    if !m.contains(&a) {
                        res += a;
                        m.insert(a);
                    }
                } else {
                    break;
                }
            }
        }
    }
    println!("{res}");
}
