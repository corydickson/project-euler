// Let d(n) be defined as the sum of proper divisors of n (numbers less than n which divide evenly into n).
// If d(a) = b and d(b) = a, where a ≠ b, then a and b are an amicable pair and each of a and b are called amicable numbers.

// For example, the proper divisors of 220 are 1, 2, 4, 5, 10, 11, 20, 22, 44, 55 and 110; therefore d(220) = 284.
// The proper divisors of 284 are 1, 2, 4, 71 and 142; so d(284) = 220.

// Evaluate the sum of all the amicable numbers under 10000.
// https://projecteuler.net/problem=21
use std::collections::HashMap;

fn find_proper_divisors(n: u128) -> Vec<u128> {
    let mut divs = std::vec![1]; // every number is divisible by 1
    for x in 2..n {
        if n % x == 0 {
            divs.push(x);
        }
    }

    return divs;
}

fn find_amicable_numbers(under: u128) -> Vec<u128> {
    // For each number less than under find all it's proper divisors
    // then have a map from number -> divsor_sum
    // now loop through that mapping and find cases of a -> b && b -> a, push both b & a
    // sort / dedup then return

    let mut sums: HashMap<u128, u128> = HashMap::new();
    sums.insert(1,1);

    for x in 2..under {
        let divs: Vec<u128> = find_proper_divisors(x);
        let sum: u128 = divs.iter().sum();
        sums.insert(x, sum);
    }

    let mut amicable: Vec<u128> = vec![];

    for key in sums.keys() {
        let value: u128 = *sums.get(key).unwrap();

        if sums.get(&value) == Some(key) && *key != value {
            amicable.push(*key);
        }
    }

    amicable.sort();
    amicable.dedup();

    return amicable;
}

fn main() {
    // Sanity check
    let pd_220: Vec<u128> = find_proper_divisors(220);
    let pd_284: Vec<u128> = find_proper_divisors(284);

    assert_eq!(pd_220, [1, 2, 4, 5, 10, 11, 20, 22, 44, 55, 110]);

    let d_220: u128 = pd_220.iter().sum();
    assert_eq!(d_220, 284);

    let d_284: u128 = pd_284.iter().sum();
    assert_eq!(d_284, 220);

    let an_300: Vec<u128> = find_amicable_numbers(300);
    println!("{:?}", an_300);

    // thanks hacker rank
    let an_300_sum: u128 = an_300.iter().sum();
    assert_eq!(an_300_sum, 504);

    // Answer:
    let an_1000: Vec<u128> = find_amicable_numbers(1000);
    println!("{:?}", an_1000);

    let an_1000_sum: u128 = an_1000.iter().sum();
    println!("Sum: {}", an_1000_sum);
}
