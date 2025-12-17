use std::io::{self, BufRead};

fn distance(a: &(f64, f64, f64), b: &(f64, f64, f64)) -> f64 {
    let (x, y, z) = *a;
    let (x2, y2, z2) = *b;
    let x3 = x - x2;
    let y3 = y - y2;
    let z3 = z - z2;
    let res = (x3 * x3 + y3 * y3 + z3 * z3).sqrt();
    res
}

#[derive(Clone)]
struct DSU {
    parent: usize,
    rank: usize,
    id: usize,
}

fn find(x: usize, dsu: &mut Vec<DSU>) -> usize {
    if dsu[x].parent == x {
        return x;
    }
    dsu[x].parent = find(dsu[x].parent, dsu);
    dsu[x].parent
}

fn union(x: usize, y: usize, dsu: &mut Vec<DSU>) -> bool {
    let x_root = find(x, dsu);
    let y_root = find(y, dsu);
    if x_root == y_root {
        return false;
    }
    if dsu[x_root].rank > dsu[y_root].rank {
        dsu[y_root].parent = x_root;
        dsu[x_root].rank += dsu[y_root].rank;
    } else {
        dsu[x_root].parent = y_root;
        dsu[y_root].rank += dsu[x_root].rank;
    }
    true
}

fn main() {
    let lines = io::stdin().lock().lines();
    let mut v = vec![];
    for line in lines {
        let line = line.unwrap();
        let mut line = line.split(',');
        let x = line.next().unwrap().parse::<f64>().unwrap();
        let y = line.next().unwrap().parse::<f64>().unwrap();
        let z = line.next().unwrap().parse::<f64>().unwrap();
        v.push((x, y, z));
    }
    let mut g: Vec<(f64, usize, usize)> = vec![];
    for i in 0..v.len() {
        for j in i + 1..v.len() {
            let dist = distance(&v[i], &v[j]);
            g.push((dist, i, j));
        }
    }
    g.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mut dsu: Vec<DSU> = vec![
        DSU {
            parent: 0,
            rank: 1,
            id: 0
        };
        v.len()
    ];
    for i in 0..dsu.len() {
        dsu[i].parent = i;
        dsu[i].id = i;
    }
    let mut steps = 1000;
    for val in &g {
        if steps == 0 {
            break;
        }
        if union(val.1, val.2, &mut dsu) {}
        steps -= 1;
    }
    dsu.sort_by_key(|k| k.rank);
    dsu.reverse();
    let mut cnt = 3;
    let mut res = 1;
    for d in &dsu {
        if d.parent == d.id {
            cnt -= 1;
            res *= d.rank;
        }
        if cnt == 0 {
            break;
        }
    }
    println!("{res}");
}
