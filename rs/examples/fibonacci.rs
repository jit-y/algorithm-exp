fn fibo(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let res = fibo(n - 1) + fibo(n - 2);
            println!("{}", res);
            res
        }
    }
}

fn memo_fibo(n: usize, memo: &mut Vec<usize>) -> usize {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            if let Some(v) = memo.get(n) {
                return *v;
            }

            let res = memo_fibo(n - 1, memo) + memo_fibo(n - 2, memo);
            memo.insert(n, res);
            res
        }
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_fibo() {
        assert_eq!(fibo(6), 0);
    }

    fn test_memo_fibo() {
        let mut memo = vec![];
        assert_eq!(memo_fibo(6, &mut memo), 0);
    }
}
