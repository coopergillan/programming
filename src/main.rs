fn main() {
    println!("Hello, world!");
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);
    assert_eq!(gcd(10, 18), 2);
    assert_eq!(gcd(20, 45), 5);
    assert_eq!(gcd(2 * 3 * 5 * 17 * 41, 5 * 19 * 41), 5 * 41)
}
