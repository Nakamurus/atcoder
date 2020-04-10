use std::io::*;
use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let stdin = stdin();
    let stdin = stdin.lock();
    let token: String = stdin
        .bytes()
        .map(|c| c.expect("failed to read char") as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect();
    token.parse().ok().expect("failed to parse token")
}

fn main() {
    let n: u32 = read();
    let a: u32 = read();
    let b: u32 = read();

    let mut ans = 0;

    let mut sum = 0;
    for mut i in 1..n+1 {
        sum = 0;
        let num = i;
        while i > 0 {
            // 10で割ったときのあまりは1桁目の数
            // 元の数を10で割り、あまり(各桁)の和を求めるループ
            sum += i % 10;
            i /= 10;
        }
        if a <= sum && sum <= b {
            // 各桁の和が条件に沿えば、元の数を加える
            ans += num;
        }
    }
    println!("{}", ans);
}