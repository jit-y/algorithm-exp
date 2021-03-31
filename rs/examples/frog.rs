fn main() {
    let a: Vec<i32> = vec![2, 9, 4, 5, 1, 6, 10];
    let mut dp = vec![std::i32::MAX; a.len()];

    dp[0] = 0;

    for i in 1..a.len() {
        if i == 1 {
            dp[i] = (a[i] - a[i - 1]).abs();
        } else {
            let one = dp[i - 1] + (a[i] - a[i - 1]).abs();
            let two = dp[i - 2] + (a[i] - a[i - 2]).abs();

            dp[i] = std::cmp::min(one, two);
        }
    }

    println!("{}", dp[a.len() - 1]);
}
