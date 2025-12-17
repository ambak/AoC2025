use microlp::{ComparisonOp, OptimizationDirection, Problem};
use std::io::{self, BufRead};

fn main() {
    let lines = io::stdin().lock().lines();
    let mut res = 0;
    for line in lines {
        let line = line.unwrap();
        let line = line.split_ascii_whitespace();
        let mut v = vec![];
        let mut joltage = vec![];
        for l in line.skip(1) {
            if l.starts_with('(') {
                let l = &l[1..l.len() - 1];
                let l = l.split(',');
                let mut schematic = 0;
                for c in l {
                    let val = c.parse::<i32>().unwrap();
                    schematic += 1 << val;
                }
                v.push(schematic);
            } else if l.starts_with('{') {
                let l = &l[1..l.len() - 1];
                let l = l.split(',');
                for (j, c) in l.enumerate() {
                    let val = c.parse::<i32>().unwrap();
                    joltage.push((val, j));
                }
            }
        }
        let mut lp_var = vec![];
        let mut problem = Problem::new(OptimizationDirection::Minimize);
        for _ in &v {
            lp_var.push(problem.add_integer_var(1., (0, 1000)));
        }
        for (val, i) in joltage {
            let mut vars = vec![];
            for (j, x) in v.iter().enumerate() {
                if x & (1 << i) != 0 {
                    vars.push((lp_var[j], 1.0));
                }
            }
            problem.add_constraint(vars, ComparisonOp::Eq, val as f64);
        }
        let solution = problem.solve().unwrap();
        res += solution.objective().round() as i32;
    }
    println!("{res}");
}
