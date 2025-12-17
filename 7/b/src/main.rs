use std::io::{self, BufRead};

fn main() {
    let lines = io::stdin().lock().lines();
    let mut v = vec![];
    for line in lines {
        v.push(line.unwrap().as_bytes().to_vec());
    }
    let mut w: Vec<Vec<u64>> = vec![vec![0; v[0].len()]; v.len()];

    for i in 0..v[0].len() {
        if v[0][i] == 'S' as u8 {
            w[0][i] = 1;
        }
    }

    let mut res = 0u64;
    for i in 1..v.len() {
        for j in 0..v[i].len() {
            if v[i - 1][j] == 'S' as u8 {
                if v[i][j] != '^' as u8 {
                    v[i][j] = 'S' as u8;
                    w[i][j] += w[i - 1][j];
                } else if v[i][j] == '^' as u8 {
                    v[i][j - 1] = 'S' as u8;
                    v[i][j + 1] = 'S' as u8;
                    w[i][j - 1] += w[i - 1][j];
                    w[i][j + 1] += w[i - 1][j];
                }
            }
        }
    }
    for i in 0..v[0].len() {
        res += w.last().unwrap()[i];
    }
    println!("{res}");
}
