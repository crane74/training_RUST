use proconio::{input, marker::Chars};
use std::collections::VecDeque;
fn main() {
    input! {
      s:Chars,
    }
    let mut v: VecDeque<char> = s.iter().map(|&x| x).collect();
    let r = v.pop_front().unwrap();
    v.push_back(r);
    for i in v.iter() {
        print!("{}", i);
    }
    println!("");
}
