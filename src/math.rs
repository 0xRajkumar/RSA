pub fn gcd(mut a: u128, mut b: u128) -> u128 {
    if b > a {
        (a, b) = (b, a);
    }
    loop {
        let r = a % b;
        a = b;
        b = r;
        if b == 0 {
            return a;
        }
    }
}
pub fn multiplicative_inverse(mut a: u128, mut b: u128) -> i128 {
    let mut t: i128;
    let mut t1: i128 = 0;
    let mut t2: i128 = 1;
    loop {
        if b == 0 {
            return t1;
        }
        let r = a % b;
        let q = a / b;
        a = b;
        b = r;
        t = t1 - (t2 * q as i128);
        t1 = t2;
        t2 = t;
    }
}
