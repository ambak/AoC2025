use std::{
    cmp::max,
    io::{self, BufRead},
};

fn main() {
    let lines = io::stdin().lock().lines();
    let mut res = 0;
    for line in lines {
        let line = line.unwrap();
        let line = line.as_bytes();
        let mut st = 0;
        let mut nd = 0;
        for i in 0..line.len() - 1 {
            let x = (&line[i] - '0' as u8) as i64;
            if x > st {
                st = x;
                nd = 0;
            } else {
                nd = max(x, nd);
            }
        }
        let x = (&line[line.len() - 1] - '0' as u8) as i64;
        nd = max(x, nd);
        res += st * 10 + nd;
    }
    println!("{res}");
}
