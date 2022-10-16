use std::str::FromStr;
use std::env;

fn main() -> () {
    let args: Vec<String> = env::args().collect();
    let mut primes: Vec<u64> = Vec::new();
    let inp: u64 = u64::from_str(&args[1]).unwrap();
    let mut c: u64 = 11;

    while c < inp  {
        if is_prime(&c) {
            primes.push(c);
        }
        c += 1;
    }

    println!("2");
    println!("3");
    println!("5");
    println!("7");
    for x in primes {
        println!("{}", x);
    }
}

fn is_prime(num: &u64) -> bool {
    
    let max: u64 = f64::sqrt(*num as f64) as u64;
    for i in 2..max+1 {
        if num % i == 0 {
            return false
        }
    }
    return true
}
