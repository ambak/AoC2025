use std::io::{self, BufRead};

fn check(x: usize, y: usize, v: &Vec<Vec<char>>) -> bool {
    if v[x][y] == '.' {
        return false;
    }
    let mut cnt = 0;
    for i in -1..=1 {
        for j in -1..=1 {
            let a = (x as i64 - i) as usize;
            let b = (y as i64 - j) as usize;
            if a == x && b == y {
                continue;
            }
            if v[a][b] != '.' {
                cnt += 1;
            }
        }
    }
    cnt < 4
}

fn main() {
    let lines = io::stdin().lock().lines();
    let mut v: Vec<Vec<char>> = vec![];
    for line in lines {
        let line = line.unwrap();
        if v.is_empty() {
            let mut w = vec![];
            for _ in 0..line.len() + 2 {
                w.push('.');
            }
            v.push(w);
        }
        let mut w = vec![];
        w.push('.');
        for c in line.chars() {
            w.push(c);
        }
        w.push('.');
        v.push(w);
    }
    let mut w = vec![];
    for _ in 0..v[0].len() {
        w.push('.');
    }
    v.push(w);

    let mut res = 0;

    for i in 1..v.len() - 1 {
        for j in 1..v[0].len() - 1 {
            if check(i, j, &v) {
                res += 1;
                v[i][j] = 'X';
            }
        }
    }

    println!("{res}");
}
