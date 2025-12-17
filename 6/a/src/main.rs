use std::io::{self, BufRead};

fn main() {
    let lines = io::stdin().lock().lines();
    let mut v: Vec<Vec<i64>> = vec![];
    let mut res = 0;
    for line in lines {
        let line = line.unwrap().trim().to_string();
        let line = line.split_ascii_whitespace();
        let mut g: Vec<i64> = vec![];
        for (i, val) in line.enumerate() {
            let num = val.parse::<i64>();
            match num {
                Ok(x) => g.push(x),
                Err(_) => {
                    if val == "*" {
                        let mut ans = 1;
                        for j in 0..v.len() {
                            ans *= v[j][i];
                        }
                        res += ans;
                    } else {
                        let mut ans = 0;
                        for j in 0..v.len() {
                            ans += v[j][i];
                        }
                        res += ans;
                    }
                }
            }
        }
        v.push(g);
    }
    println!("{res}");
}
