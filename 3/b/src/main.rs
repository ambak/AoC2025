use std::{
    cmp::min,
    io::{self, BufRead},
};

fn main() {
    let lines = io::stdin().lock().lines();
    let mut res = 0;
    let size = 12;
    for line in lines {
        let line = line.unwrap();
        let line = line.as_bytes();
        let mut v = vec![0i64; size];
        for i in 0..line.len() {
            let x = (&line[i] - '0' as u8) as i64;
            for j in 0..min(size, line.len() - i) {
                if x > v[j] {
                    v[j] = x;
                    if j > 0 {
                        v[j - 1] = 0;
                    }
                }
            }
        }
        let mut m = 1;
        for val in v {
            res += val * m;
            m *= 10;
        }
    }
    println!("{res}");
}
