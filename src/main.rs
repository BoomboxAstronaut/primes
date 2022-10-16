use std::str::FromStr;
use std::env;

fn main() -> () {
    let args: Vec<String> = env::args().collect();
    let mut primes: Vec<f64> = Vec::from([2.0, 3.0, 5.0, 7.0]);
    let inp: u64 = u64::from_str(&args[1]).unwrap();
    let mut i: u64 = 11;

    while i < inp  {
        if is_prime(&i, &primes) {
            primes.push(i as f64);
        }
        i += 1;
    }
   
    for x in primes {
        println!("{}", x);
    }
}

fn is_prime(num: &u64, plist: &Vec<f64>) -> bool {
    
    if num & 1 == 0 {
        return false
    }
    if num % 7 == 0 {
        return false
    }
    
    let inv = num.to_string();
    let inv: Vec<u64> = inv.split("").filter_map(|x| u64::from_str(x).ok()).collect();
    let mut c3: u64 = 0;

    if *inv.last().unwrap() == 5u64 {
        return false
    }
    for x in &inv {
        c3 = c3 + *x;
    }
    if c3 % 3 == 0 {
        return false
    }
    
    let cmp: f64 = f64::sqrt(*num as f64);
    let below: Vec<&f64> = plist.iter().filter(|x| **x <= cmp).collect();

    let num: f64 = *num as f64;

    for x in below {
        if num % x == 0.0 {
            return false
        }
    }
    return true
}
