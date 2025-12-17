use std::io::{self, BufRead};

fn main() {
    let lines = io::stdin().lock().lines();
    let mut v = vec![];
    let mut cnt = 0;
    let mut res = 0;
    for line in lines {
        let line = line.unwrap();
        if line.contains('x') {
            let mut s = line.split_whitespace();
            let mut expression = s.next().unwrap().split('x');
            let dim1 = expression.next().unwrap().parse::<i32>().unwrap();
            let dim2 = expression
                .next()
                .unwrap()
                .trim_end_matches(':')
                .parse::<i32>()
                .unwrap();
            let mut gifts_size = 0;
            let mut gifts_count = 0;
            for (i, x) in s.enumerate() {
                let x = x.parse::<i32>().unwrap();
                gifts_count += x;
                gifts_size += v[i] * x;
            }
            if (dim1 / 3) * (dim2 / 3) >= gifts_count {
                res += 1;
            } else if dim1 * dim2 < gifts_size {
            } else {
                panic!();
            }
        } else {
            if line == "" {
                v.push(cnt);
            } else {
                cnt += line.chars().filter(|c| *c == '#').count() as i32;
            }
        }
    }
    println!("{res}");
}
