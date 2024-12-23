use std::collections::{HashMap, HashSet};
use itertools::Itertools;
use crate::helpers::read_lines;


fn prune(val: i64) -> i64 {
    val % 16777216
}

fn mix(v0: i64, v1: i64) -> i64 {
    v0 ^ v1
}

fn step(val: i64) -> i64 {
    let v0 = prune(mix(val, val * 64));
    let v1 = prune(mix(v0, v0 / 32));
    let v2 = prune(mix(v1, v1 * 2048));
    return v2;
}

fn compute_one(init: i64, steps: i64) -> i64 {
    let mut res = init;
    for i in 0..steps {
        res = step(res);
    }
    res
}

fn collect_one(init: i64, steps: i64) -> Vec<i32> {
    let mut result = vec![(init % 10) as i32];
    let mut res = init;
    for i in 0..steps {
        res = step(res);
        result.push((res % 10) as i32);
    }
    result
}

fn compute(nums: &Vec<i64>, steps: i64) -> i64 {
    let mut result = 0;
    for &num in nums {
        let res = compute_one(num, steps);
        println!("{}: {:?}", num, res);
        result += res;
    }
    result
}

fn adjacent_differences(vec: &[i32]) -> Vec<i32> {
    vec.windows(2).map(|w| w[1] - w[0]).collect()
}

type Sec = (i32, i32, i32, i32);
type SecToBan = HashMap<Sec, i32>;

fn compute_sec_to_ban(prices: &Vec<i32>) -> SecToBan {
    let mut result: SecToBan = HashMap::new();
    let adjacent = adjacent_differences(prices);
    for idx in 4..prices.len() {
        let price = prices[idx];
        let key = (adjacent[idx - 4], adjacent[idx - 3], adjacent[idx - 2], adjacent[idx - 1]);
        if result.contains_key(&key) { continue; }
        result.insert(key, price);
    }
    result
}

fn find_best_pattern(sec_to_bans: &Vec<SecToBan>) -> (Sec, i32) {
    println!("Computing all sequences.");
    //let all_secs: HashSet<Sec> = sec_to_bans.iter().fold(HashSet::<Sec>::new(), |mut s, sb| {s.extend(sb.keys()); s});
    //println!("Num sequences: {}", all_secs.len());
    let mut best_seq: Sec = (-1, -1, -1, -1);
    let mut best_price = -1;
    println!("Finding best sequence.");
    //for seq in all_secs {
    for a in -9..10 {
        println!("a: {a}");
        for b in -9..10 {
            for c in -9..10 {
                for d in -9..10 {
                    let seq = (a, b, c, d);
                    let price = sec_to_bans.iter()
                        .map(|sb| sb.get(&seq).unwrap_or(&0)) // Get prices for this sequence.
                        .fold(0, |acc, price| acc + price); // Sum up the prices.
                    if price > best_price {
                        best_price = price;
                        best_seq = seq;
                    }
                }
            }
        }
    }
    (best_seq, best_price)
}

fn compute2(nums: &Vec<i64>, steps: i64) -> i64 {
    println!("Computing prices.");
    let all_prices = nums.iter().map(|&num| collect_one(num, steps)).collect_vec();
    println!("Computing sec_to_bans.");
    let sec_to_bans: Vec<SecToBan> = all_prices.iter().map(|prices| compute_sec_to_ban(prices)).collect_vec();
    println!("Finding best pattern.");
    let (seq, price) = find_best_pattern(&sec_to_bans);
    println!("{:?} {:?}", seq, price);
    price as i64
}

#[allow(dead_code)]
pub(crate) fn dec22() {
    let mut res = 123;
    for i in 0..10 {
        res = step(res);
        println!("{:?}", res);
    }
    let lines = read_lines("dec22.in.txt").expect("Could not load input.");
    let nums: Vec<i64> = lines.iter().map(|l| l.parse().unwrap()).collect();
    let result = compute(&nums, 2000);
    println!("{:?}", result);
}
#[allow(dead_code)]
pub(crate) fn dec22_2() {
    let lines = read_lines("dec22.in.txt").expect("Could not load input.");
    let nums: Vec<i64> = lines.iter().map(|l| l.parse().unwrap()).collect();
    let result = compute2(&nums, 2000);
    println!("{:?}", result);
}
