#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        N: usize,
        A: [i32; N],
    }

    let mut ans = std::i32::MAX;

    for i in 0..(1 << (N - 1)) {
        let mut xored = 0;
        let mut ored = 0;
        for j in 0..N {
            ored |= A[j];
            if (i >> j) & 1 == 1 {
                xored ^= ored;
                ored = 0;
            }
        }
        xored ^= ored;
        ans = ans.min(xored);
    }

    println!("{}", ans);
}
