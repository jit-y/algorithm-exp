fn rec(i: usize, dp: &mut [i32], a: &[i32]) -> i32 {
    if dp[i] < std::i32::MAX {
        return dp[i];
    }

    if i == 0 {
        return 0;
    }

    let mut res = std::i32::MAX;

    res = std::cmp::min(res, rec(i - 1, dp, a) + (a[i] - a[i - 1]).abs());
    if i > 1 {
        res = std::cmp::min(res, rec(i - 2, dp, a) + (a[i] - a[i - 2]).abs());
    }

    dp[i] = res;

    dp[i]
}

fn main() {
    let a: Vec<i32> = vec![2, 9, 4, 5, 1, 6, 10];
    let mut dp = vec![std::i32::MAX; a.len()];

    let res = rec(a.len() - 1, &mut dp, &a);

    println!("{}", res);
}
