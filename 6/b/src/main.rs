use std::io::{self, BufRead};

fn main() {
    let lines = io::stdin().lock().lines();
    let mut v: Vec<Vec<u8>> = vec![];
    let mut res = 0;
    for line in lines {
        let line = line.unwrap().as_bytes().to_vec();
        v.push(line);
    }
    let mut ans = 0;
    let mut mul = true;
    for i in 0..v[0].len() {
        let mut x = 0;
        let mut ok = false;
        for j in 0..v.len() - 1 {
            if v[j][i] != ' ' as u8 {
                ok = true;
                x += (v[j][i] - '0' as u8) as i64;
                x *= 10;
            }
        }
        x /= 10;
        if v.last().unwrap()[i] == '*' as u8 {
            mul = true;
            ans = 1;
        } else if v.last().unwrap()[i] == '+' as u8 {
            mul = false;
            ans = 0;
        }
        if !ok {
            res += ans;
        } else if mul {
            ans *= x;
        } else {
            ans += x;
        }
    }
    res += ans;
    println!("{res}");
}
