use std::{
    cmp::min,
    io::{self, BufRead},
};

fn solve(step: i32, id: usize, val: i32, lights: i32, v: &Vec<i32>) -> Option<i32> {
    if val == lights {
        return Some(step);
    }
    let mut res = v.len() as i32;
    for i in id..v.len() {
        match solve(step + 1, i + 1, val ^ v[i], lights, v) {
            Some(x) => res = min(res, x),
            None => (),
        }
    }
    if res < v.len() as i32 {
        return Some(res);
    }
    None
}

fn main() {
    let lines = io::stdin().lock().lines();
    let mut res = 0;
    for line in lines {
        let line = line.unwrap();
        let mut line = line.split_ascii_whitespace();
        let mut id = 0;
        let mut lights = 0;
        let mut v = vec![];
        let l = line.next().unwrap();
        for c in l.chars() {
            if c == '#' {
                lights += 1 << id;
            }
            if c == '.' || c == '#' {
                id += 1;
            }
        }
        for l in line {
            if l.starts_with('(') {
                let l = &l[1..l.len() - 1];
                let l = l.split(',');
                let mut schematic = 0;
                for c in l {
                    let val = c.parse::<i32>().unwrap();
                    schematic += 1 << val;
                }
                v.push(schematic);
            }
        }
        match solve(0, 0, 0, lights, &v) {
            Some(x) => res += x,
            None => (),
        }
    }
    println!("{res}");
}
