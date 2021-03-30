fn partial_sum(i: usize, w: i32, a: &[i32]) -> bool {
    if i == 0 {
        let res = if w == 0 { true } else { false };

        return res;
    }

    if partial_sum(i - 1, w, a) {
        return true;
    }

    if partial_sum(i - 1, w - a[i - 1], a) {
        return true;
    }

    false
}

fn main() {
    let a = vec![3, 2, 6, 5];
    println!("{}", partial_sum(a.len(), 14, &a));
}
