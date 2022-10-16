use std::str::FromStr;
use std::env;

fn main() -> () {
    let args: Vec<String> = env::args().collect();
    let mut primes: Vec<u64> = Vec::new();
    let inp: u64 = u64::from_str(&args[1]).unwrap();
    let mut i: u64 = 11;

    while i < inp  {
        if is_prime(&i, &primes) {
            primes.push(i);
        }
        i += 1;
    }

    println!("2");
    println!("3");
    println!("5");
    println!("7");
    for x in primes {
        println!("{}", x);
    }
}

fn is_prime(num: &u64, plist: &Vec<u64>) -> bool {
    
    if num & 1 == 0 {
        return false
    }
    if num % 7 == 0 {
        return false
    }
    if num % 3 == 0 {
        return false
    }
    if num.to_string().chars().last().unwrap() == '5' {
        return false
    }
    
    let cmp: u64 = f64::sqrt(*num as f64) as u64;
    let below: Vec<&u64> = plist.iter().filter(|x| **x <= cmp).collect();

    for x in below {
        if num % x == 0 {
            return false
        }
    }
    return true
}
