fn gdc(m: u32, n: u32) -> u32 {
    if n == 0 {
        return m;
    }

    gdc(n, m % n)
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_gdc() {
        assert_eq!(gdc(51, 15), 3);
        assert_eq!(gdc(15, 51), 3);
    }
}
