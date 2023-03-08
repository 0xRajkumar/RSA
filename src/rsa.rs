use crate::math;
use primal::is_prime;
use rand::Rng;
use std::io;
pub fn gen() {
    let p: u64 = loop {
        let p = rand::thread_rng().gen_range(1..=10000);
        let isprime = is_prime(p.into());
        if isprime {
            break p;
        }
    };
    let q: u64 = loop {
        let q = rand::thread_rng().gen_range(1..=10000);
        let isprime = is_prime(q);
        if isprime {
            break q;
        }
    };
    println!("n is {}", p * q);
    println!("P AND Q ARE {},{}", p, q);
    let phi = (p as u128 - 1) * (q as u128 - 1);
    println!("PHI is {}", phi);
    let e = loop {
        let e = rand::thread_rng().gen_range(1..=phi);
        let gcdis = math::gcd(phi, e);
        if gcdis == 1 {
            break e;
        }
    };
    println!("E is {}", e);
    let mut d = math::multiplicative_inverse(phi, e);
    if d < 0 {
        d = d + phi as i128;
    }
    println!("D is {}", d);
}

fn encrypt(m: u128, mut e: u128, n: u128) -> u128 {
    let mut c: u128 = 1;
    loop {
        if e == 0 {
            return c;
        }
        c = c * m % n;
        e = e - 1;
    }
}
fn decrypt(c: u128, mut d: u128, n: u128) -> u128 {
    let mut m: u128 = 1;
    loop {
        if d == 0 {
            return m;
        }
        m = m * c % n;
        d = d - 1;
    }
}
pub fn en_fun() {
    let mut m = String::new();
    println!("M");
    io::stdin().read_line(&mut m).expect("Failed to read line");
    let mn: u128 = m.trim().parse().expect("Please type a number!");

    let mut e = String::new();
    println!("E");
    io::stdin().read_line(&mut e).expect("Failed to read line");
    let en: u128 = e.trim().parse().expect("Please type a number!");

    let mut n = String::new();
    println!("N");
    io::stdin().read_line(&mut n).expect("Failed to read line");
    let nn: u128 = n.trim().parse().expect("Please type a number!");
    println!("c is {}", encrypt(mn, en, nn));
}
pub fn de_fun() {
    let mut c = String::new();
    println!("C");
    io::stdin().read_line(&mut c).expect("Failed to read line");
    let cn: u128 = c.trim().parse().expect("Please type a number!");

    println!("D");
    let mut d = String::new();
    io::stdin().read_line(&mut d).expect("Failed to read line");
    let dn: u128 = d.trim().parse().expect("Please type a number!");

    println!("N");
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read line");
    let nn: u128 = n.trim().parse().expect("Please type a number!");
    println!("m is {}", decrypt(cn, dn, nn));
}
