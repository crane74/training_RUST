#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        h : i8,
        w : i8,
        x : i8,
        y : i8,
        s :[String; h]
    }

    let mut ans = 1;

    let dx = [0, 1, 0, -1];
    let dy = [-1, 0, 1, 0];

    for d in 0..4 {
        let (X, Y) = (x - 1, y - 1);
        let (mut nx, mut ny) = (X + dx[d], Y + dy[d]);

        loop {
            if nx < 0 || nx >= h || ny < 0 || ny >= w {
                break;
            }
            if s[nx as usize].chars().nth(ny as usize).unwrap() == '#' {
                break;
            }
            ans += 1;
            nx += dx[d];
            ny += dy[d];
        }
    }

    println!("{}", ans);
}
