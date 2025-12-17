use std::{
    collections::{HashMap, HashSet},
    io::{self, BufRead},
};

fn bfs(g: &HashMap<String, Vec<String>>, od: &str, out: &str) -> i64 {
    let mut visited = HashSet::new();
    let mut queue = HashSet::new();
    queue.insert(od.to_string());
    visited.insert(od.to_string());
    while !queue.is_empty() {
        let x = queue.iter().cloned().next().unwrap();
        queue.remove(&x.clone());
        visited.insert(x.clone());
        if x != "out".to_string() {
            for val in &g[&x] {
                if !visited.contains(val) {
                    queue.insert(val.to_string());
                }
            }
        }
    }

    let mut q: HashMap<String, i64> = HashMap::new();
    q.insert(od.to_string(), 1);

    let mut ine: HashMap<&String, (i64, i64)> = HashMap::new();
    for (k, v) in g {
        if visited.contains(k) {
            for s in v {
                ine.entry(s)
                    .and_modify(|x: &mut (i64, i64)| x.0 += 1)
                    .or_insert((1, 0));
            }
        }
    }

    while !q.is_empty() {
        let x = q.keys().cloned().next().unwrap();
        let y = q.remove(&x.clone()).unwrap();
        if *x == out.to_string() {
            continue;
        }
        for val in &g[&x] {
            ine.entry(val)
                .and_modify(|x: &mut (i64, i64)| {
                    x.0 -= 1;
                    x.1 += y;
                })
                .or_insert((0, 0));
            if ine[val].0 == 0 {
                q.insert(val.to_string(), ine[val].1);
            }
        }
    }
    if ine.contains_key(&out.to_string()) {
        return ine[&out.to_string()].1;
    }
    0
}

fn main() {
    let lines = io::stdin().lock().lines();
    let mut g: HashMap<String, Vec<String>> = HashMap::new();
    for line in lines {
        let line = line.unwrap();
        let mut line = line.split_whitespace();
        let mut a = line.next().unwrap().to_string();
        a.pop();
        let mut v = vec![];
        for x in line.into_iter() {
            v.push(x.to_string());
        }
        g.insert(a, v);
    }
    g.insert("out".to_string(), Vec::<String>::new());
    println!("{}", bfs(&g, "you", "out"));
}
