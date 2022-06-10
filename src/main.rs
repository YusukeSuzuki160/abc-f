use num::integer;
use proconio::input;
use std::process;

struct SegTree {
    size: usize,
    tree: Vec<usize>,
}

impl SegTree {
    fn new(n: usize) -> Self {
        let mut x = 1;
        while x < n {
            x *= 2;
        }
        let mut tree = Vec::new();
        for _i in 0..(2 * x - 1) {
            tree.push(0);
        }
        SegTree {
            size: x,
            tree: tree,
        }
    }
    fn set_tree(&mut self, list: &Vec<usize>) {
        for i in 0..list.len() {
            self.tree[i + self.size - 1] = list[i];
        }
        let mut len = self.size / 2;
        while len >= 1 {
            for i in 0..len {
                if self.tree[2 * (i + len - 1) + 1] == 0 {
                    self.tree[i + len - 1] = self.tree[2 * (i + len - 1) + 2];
                } else if self.tree[2 * (i + len - 1) + 2] == 0 {
                    self.tree[i + len - 1] = self.tree[2 * (i + len - 1) + 1];
                } else {
                    self.tree[i + len - 1] = integer::gcd(
                        self.tree[2 * (i + len - 1) + 1],
                        self.tree[2 * (i + len - 1) + 2],
                    );
                }
            }
            len /= 2;
        }
    }
    fn gcd(&self, i: usize, j: usize, k: usize, n: usize, m: usize) -> usize {
        if m <= i || j <= n {
            0
        } else if i <= n && m <= j {
            self.tree[k]
        } else {
            let v1 = self.gcd(i, j, k * 2 + 1, n, (n + m) / 2);
            let v2 = self.gcd(i, j, k * 2 + 2, (n + m) / 2, m);
            if v1 == 0 {
                v2
            } else if v2 == 0 {
                v1
            } else {
                integer::gcd(v1, v2)
            }
        }
    }
    fn get_gcd(&self, i: usize, j: usize) -> usize {
        self.gcd(i, j, 0, 0, self.size)
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [isize; n],
        b: [isize; n],
        query: [(usize, usize, usize, usize); q],
    }
    let mut sub_a: Vec<usize> = Vec::new();
    let mut sub_b: Vec<usize> = Vec::new();
    if n == 1 {
        for _i in 0..q {
            println!("{}", a[0] + b[0]);
        }
        process::exit(0);
    }
    for i in 0..(n - 1) {
        sub_a.push((a[i + 1] - a[i]).abs() as usize);
        sub_b.push((b[i + 1] - b[i]).abs() as usize);
    }
    let mut a_tree = SegTree::new(n - 1);
    let mut b_tree = SegTree::new(n - 1);
    a_tree.set_tree(&sub_a);
    b_tree.set_tree(&sub_b);
    for (h_1, h_2, w_1, w_2) in query {
        let base = (a[h_1 - 1] + b[w_1 - 1]) as usize;
        let mut gcd_a = a_tree.get_gcd(h_1 - 1, h_2 - 1);
        if h_1 == h_2 {
            gcd_a = base;
        }
        let mut gcd_b = b_tree.get_gcd(w_1 - 1, w_2 - 1);
        if w_1 == w_2 {
            gcd_b = base;
        }
        println!("{}", integer::gcd(integer::gcd(base, gcd_a), gcd_b));
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use num::integer;
    use rand::Rng;
    use std::mem;
    fn solve(n: usize, a: &Vec<isize>, b: &Vec<isize>, query: (usize, usize, usize, usize)) -> usize {
        let mut list = Vec::new();
        for _i in 0..n {
            list.push(Vec::new());
        }
        for i in 0..n {
            for j in 0..n {
                list[i].push((a[i] + b[j]) as usize);
            }
        }
        let (h_1, h_2, w_1, w_2) = query;
        let mut ans = list[h_1 - 1][w_1 - 1];
        for i in h_1..=h_2 {
            for j in w_1..=w_2 {
                ans = integer::gcd(list[i - 1][j - 1], ans);
            }
        }
        return ans;
    }
    fn solve_2(n: usize, a: &Vec<isize>, b: &Vec<isize>, query: (usize, usize, usize, usize)) -> usize {
        let mut sub_a = Vec::new();
        let mut sub_b = Vec::new();
        if n == 1 {
            return (a[0] + b[0]) as usize;
        }
        for i in 0..(n - 1) {
            sub_a.push((a[i + 1] - a[i]).abs() as usize);
            sub_b.push((b[i + 1] - b[i]).abs() as usize);
        }
        let mut a_tree = SegTree::new(n - 1);
        let mut b_tree = SegTree::new(n - 1);
        a_tree.set_tree(&sub_a);
        b_tree.set_tree(&sub_b);
        let (h_1, h_2, w_1, w_2) = query;
        let base = (a[h_1 - 1] + b[w_1 - 1]) as usize;
        let mut gcd_a = a_tree.get_gcd(h_1 - 1, h_2 - 1);
        if h_1 == h_2 {
            gcd_a = base;
        }
        let mut gcd_b = b_tree.get_gcd(w_1 - 1, w_2 - 1);
        if w_1 == w_2 {
            gcd_b = base;
        }
        return integer::gcd(integer::gcd(base, gcd_a), gcd_b);
    }
    #[test]
    fn test() {
        let mut rng = rand::thread_rng();
        let n: usize = rng.gen::<usize>() % (2 * 10_usize.pow(1)) + 6;
        let mut a = Vec::new();
        let mut b = Vec::new();
        for _i in 0..n {
            let a_add: usize = rng.gen::<usize>() % 10_usize.pow(3) + 1;
            let b_add: usize = rng.gen::<usize>() % 10_usize.pow(3) + 1;
            a.push(a_add as isize);
            b.push(b_add as isize);
        }
        /*let mut h_1 = rng.gen::<usize>() % n + 1;
        let mut h_2 = rng.gen::<usize>() % n + 1;
        if h_1 > h_2 {
            mem::swap(&mut h_1, &mut h_2);
        }
        let mut w_1 = rng.gen::<usize>() % n;
        let mut w_2 = rng.gen::<usize>() % n;
        if w_1 > w_2 {
            mem::swap(&mut w_1, &mut w_2);
        }*/
        for i in &a {
            print!("{}, ", i);
        }
        print!("\n");
        for j in &b {
            print!("{}, ", j);
        }
        print!("\n");
        //println!("{}, {}, {}, {}", h_1, h_2, w_1, w_2);
        //let ans_1 = solve(n, &a, &b, (h_1, h_2, w_1, w_2));
        //let ans_2 = solve_2(n, &a, &b, (h_1, h_2, w_1, w_2));
        let ans_3 = solve(n, &a, &b, (2, 3, 4, 5));
        let ans_4 = solve_2(n, &a, &b, (2, 3, 4, 5));
        //assert_eq!(ans_1, ans_2);
        assert_eq!(ans_3, ans_4);
    }
}
