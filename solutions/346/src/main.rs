// The number 7 is special, because 7 is 111 written in base 2, and 11 written in base 6
// (i.e. 710 = 116 = 1112). In other words, 7 is a repunit in at least two bases b > 1.
// We shall call a positive integer with this property a strong repunit.
// It can be verified that there are 8 strong repunits below 50: {1,7,13,15,21,31,40,43}.
// Furthermore, the sum of all strong repunits below 1000 equals 15864.
//
// Find the sum of all strong repunits below 10^12.
//
// https://projecteuler.net/problem=346

extern crate num;

use num::pow;

fn strong_repunits(under:u128) -> Vec<u128> {
    let mut repunits = std::vec![1];
    let mut base: u128 = 2;
    while base + pow(base, 2) < under {
        let mut repunit = base + pow(base, 2) + 1;
        while repunit < under {
            repunits.push(repunit);
            repunit *= base;
            repunit += 1;
        }

        base += 1;
    }

    repunits.sort();
    repunits.dedup();
    return repunits;
}

fn main() {
    // Sanity check
    let test = strong_repunits(50);
    assert_eq!(test, [1,7,13,15,21,31,40,43]);

    let sumtest: Vec<u128> = strong_repunits(1000);
    let sum: u128 = sumtest.iter().sum();
    assert_eq!(sum, 15864);

    // Answer:
    let solution_set: Vec<u128> = strong_repunits(pow(10, 12));
    let answer: u128 = solution_set.iter().sum();
    println!("{}", answer);
}
