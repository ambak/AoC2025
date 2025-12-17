use std::{
    cmp::{max, min},
    io::{self, BufRead},
};

fn q_remove(a: i64, b: i64, q: &mut Vec<(i64, i64, bool)>) {
    let mut ok = false;
    let mut id = q.len();
    for (i, val) in q.iter().enumerate() {
        if val.0 > a && val.1 > b {
            ok = true;
            id = min(id, i);
        }
    }
    while ok && q.len() > id {
        q.pop();
    }
}

fn max_area(id: usize, v: &Vec<(i64, i64)>, right_close: bool) -> i64 {
    let mut res = 0;
    let x = v[id].0;
    let y = v[id].1;
    let mut q: Vec<(i64, i64, bool)> = vec![];
    let mut x_only = false;
    let mut y_only = false;
    let mut prev = v[id];
    let mut prev2 = prev;
    let mut rcheck_ok = false;
    let mut r_till_end = -4;
    if right_close {
        r_till_end = 4;
    }
    for i in 1..v.len() {
        if i > 1 {
            rcheck_ok = true;
        }
        let i = (id + i) % v.len();
        let a = v[i].0;
        let b = v[i].1;
        let mut right_side = false;
        if rcheck_ok {
            right_side = check_right_side(prev2, prev, v[i]);
            if right_side {
                r_till_end -= 1;
            } else {
                r_till_end += 1;
            }
        }

        if x_only && b <= y && !q.is_empty() && q.last().unwrap().1 <= y {
            prev2 = prev;
            prev = v[i];
            continue;
        }
        if y_only && a <= x && !q.is_empty() && q.last().unwrap().0 <= x {
            prev2 = prev;
            prev = v[i];
            continue;
        }
        if q.is_empty() {
            if x < a && y < b {
                if (!right_side && r_till_end < 0) || (right_side && r_till_end > 0) {
                    q.push((v[i].0, v[i].1, true));
                } else {
                    q.push((v[i].0, v[i].1, false));
                    return 1;
                }
                if prev.0 == a {
                    y_only = true;
                } else {
                    x_only = true;
                }
            }
        } else {
            q_remove(a, b, &mut q);
            while !q.is_empty()
                && ((q.last().unwrap().0 > a && q.last().unwrap().1 > b)
                    || (prev.1 > y
                        && prev.1 < q.last().unwrap().1
                        && prev.0 < x
                        && a > q.last().unwrap().0))
            {
                q.pop();
            }
            if q.is_empty() {
                if (!right_side && r_till_end < 0) || (right_side && r_till_end > 0) {
                    q.push((v[i].0, v[i].1, true));
                } else {
                    q.push((v[i].0, v[i].1, false));
                }
                prev2 = prev;
                prev = v[i];
                continue;
            }
            if x_only {
                if b > q.last().unwrap().1 {
                    prev2 = prev;
                    prev = v[i];
                    continue;
                }
                while !q.is_empty() && q.last().unwrap().0 > a {
                    q.pop();
                }
                if (!right_side && r_till_end < 0) || (right_side && r_till_end > 0) {
                    q.push((v[i].0, v[i].1, true));
                } else {
                    q.push((v[i].0, v[i].1, false));
                }
                if q.is_empty() {
                    return 1;
                }
            }
            if y_only {
                if a > q.last().unwrap().0 {
                    prev2 = prev;
                    prev = v[i];
                    continue;
                }
                while !q.is_empty() && q.last().unwrap().1 > b {
                    q.pop();
                }
                if check_right_side(prev2, prev, v[i]) == right_close {
                    q.push((v[i].0, v[i].1, true));
                } else {
                    q.push((v[i].0, v[i].1, false));
                }
                if q.is_empty() {
                    return 1;
                }
            }
        }
        prev2 = prev;
        prev = v[i];
    }
    for (a, b, _) in q {
        if (a - x + 1) < 0 || (b - y + 1) < 0 {
            continue;
        }
        res = max(res, (a - x + 1) * (b - y + 1));
    }
    return res;
}

fn is_up_down(st: (i64, i64), nd: (i64, i64)) -> i64 {
    if st.0 == nd.0 {
        if st.1 < nd.1 {
            return 1;
        }
        return -1;
    }
    return 0;
}

fn is_right_left(st: (i64, i64), nd: (i64, i64)) -> i64 {
    if st.1 == nd.1 {
        if st.0 < nd.0 {
            return 1;
        }
        return -1;
    }
    return 0;
}

fn check_right_side(a: (i64, i64), b: (i64, i64), c: (i64, i64)) -> bool {
    let up_down1 = is_up_down(a, b);
    let up_down2 = is_up_down(b, c);
    let right_left1 = is_right_left(a, b);
    let right_left2 = is_right_left(b, c);
    if (up_down1 == 1 && right_left2 == 1)
        || (up_down1 == -1 && right_left2 == -1)
        || (right_left1 == 1 && up_down2 == -1)
        || (right_left1 == -1 && up_down2 == 1)
    {
        return true;
    }
    false
}

fn check_right_close(v: &Vec<(i64, i64)>) -> bool {
    let mut cntr = 0;
    for i in 2..v.len() {
        if check_right_side(v[i - 2], v[i - 1], v[i]) {
            cntr += 1;
        } else {
            cntr -= 1;
        }
    }
    return cntr > 0;
}

fn main() {
    let lines = io::stdin().lock().lines();
    let mut v = vec![];
    for line in lines {
        let line = line.unwrap();
        let mut line = line.split(',');
        let x = line.next().unwrap().parse::<i64>().unwrap();
        let y = line.next().unwrap().parse::<i64>().unwrap();
        v.push((x, y));
    }
    let right_close = check_right_close(&v);
    let mut res = 0;
    for i in 0..v.len() {
        res = max(res, max_area(i, &v, right_close));
    }
    for i in 0..v.len() {
        let a = -v[i].0;
        let b = v[i].1;
        v[i] = (a, b);
    }
    let right_close = check_right_close(&v);
    for i in 0..v.len() {
        res = max(res, max_area(i, &v, right_close));
    }
    println!("{res}");
}
