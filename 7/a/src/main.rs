use std::io::{self, BufRead};

fn main() {
    let lines = io::stdin().lock().lines();
    let mut v = vec![];
    for line in lines {
        v.push(line.unwrap().as_bytes().to_vec());
    }
    let mut res = 0;
    for i in 1..v.len() {
        for j in 0..v[i].len() {
            if v[i - 1][j] == 'S' as u8 {
                if v[i][j] == '.' as u8 {
                    v[i][j] = 'S' as u8;
                } else if v[i][j] == '^' as u8 {
                    res += 1;
                    v[i][j - 1] = 'S' as u8;
                    v[i][j + 1] = 'S' as u8;
                }
            }
        }
    }
    println!("{res}");
}
