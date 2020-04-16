use std::io::*;
use std::str::FromStr;

fn read<T: FromStr>() -> T {

    let stdin = stdin();
    let stdin = stdin.lock();
    let token: String = stdin
        .bytes()
        .map(|c| c.expect("Failed to read char") as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect();

    token.parse().ok().expect("failed to parse token")

}


// fn main () {
//     let n: u32 = read();
//     let s: Vec<_> = read::<String>().chars().collect();
//     let mut vec = vec![0usize; 3];
//     for &c in &s {
//         match c {
//             'R' => vec[0] += 1,
//             'G' => vec[1] += 1,
//             'B' => vec[2] += 1,
//             _ => ()
//         }
//     }

//     let mut count = 0usize;
//     for l in 1..n {
//         for i in 0..n {
//             if i + 2 * l > n - 1 {
//                 break;
//             }
//             let j = i + 1;
//             let k = i + 2 * l;
//             let si = s[i];
//             let sj = s[j];
//             let sk = s[k];
//             if si != sj && sj != sk && sk != si {
//                 count += 1;
//             }
//         }
//     }
//     println!("{}", vec[0] * vec[1] * vec[2] - count);

//     // let mut ans = 0;
//     // for i in 0..n {
//     //     for j in 0..n {
//     //         for k in 0..n {
//     //             if j - i != k - j {
//     //                 if k > j && j > i {
//     //                     let i = i as usize;
//     //                     let j = j as usize;
//     //                     let k = k as usize;
//     //                     if chars[i] !=chars[j] && chars[i] != chars[k] && chars[j] != chars[k] {
//     //                         ans += 1;
//     //                     }
//     //                 }
//     //             }
//     //         }
//     //     }
//     // }
//     // println!("{}", ans);
// }

fn main() {
    let n: u32 = read();
    let v: Vec<_> = read::<String>().chars().collect();
    let mut s: Vec<u32> = Vec::new();
    let mut ans = 1;
    ans *= v.iter().filter(|&n| *n == 'R').count();
    ans *= v.iter().filter(|&n| *n == 'G').count();
    ans *= v.iter().filter(|&n| *n == 'B').count();

    for &c in &v {
        match c {
            'R' => s.push(1),
            'G' => s.push(2),
            'B' => s.push(4),
            _ => ()
        }
    }

    for i in 0..n {
        for j in i+1..n {
            let k = 2*j-i; // j−i≠k−jから、条件を満たさないk
            if k >= n {
                continue
            }
            if s[i as usize] + s[j as usize] + s[k as usize] == 7 {
                ans -= 1;
            }
        }
    }
    println!("{}", ans);
}