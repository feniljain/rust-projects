//use std::{collections::HashMap, fs};
//
//type AnyError = Box<dyn std::error::Error>;
//type Result<T> = std::result::Result<T, AnyError>;
//
//const TOTAL_SUM: i32 = 2020;
//
//fn main() -> Result<()> {
//    let filename = "d1input.txt";
//
//    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
//
//    let numbers: Vec<i32> = contents
//        .split('\n')
//        .filter_map(|x| {
//            if let Ok(x) = x.parse::<i32>() {
//                Some(x)
//            } else {
//                None
//            }
//        })
//        .collect();
//
//    let mut num_counts = HashMap::<i32, u32>::new();
//    for &num in &numbers {
//        *num_counts.entry(num).or_default() += 1;
//    }
//
//    let mut result_q1 = 0;
//    for &x in &numbers {
//        let y = TOTAL_SUM - x;
//        if let Some(&count) = num_counts.get(&y) {
//            if (count > 1) || (count == 1 && x != y) {
//                result_q1 = x * y;
//                break;
//            }
//        }
//    }
//
//    println!("Q1 ans = {}", result_q1);
//
//    let mut result_q2 = 0;
//    for (idx, &x) in numbers.iter().enumerate() {
//        for &y in numbers.iter().skip(idx + 1) {
//            let z = TOTAL_SUM - x - y;
//
//            if let Some(&count) = num_counts.get(&z) {
//                if (count > 2)
//                    || (count == 2 && (z != x || z != y))
//                    || (count == 1 && z != x && z != y)
//                {
//                    println!("{} {}, {}", x, y, z);
//                    result_q2 = x * y * z;
//                    break;
//                }
//            }
//        }
//    }
//
//    println!("Q2 ans = {}", result_q2);
//
//    Ok(())
//}

//558 823, 639
//823 639, 558
//Q2 ans = 293450526

use std::fs;

fn main() {
    let filename = "d1input.txt";

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut nos: Vec<i32> = contents
        .split('\n')
        .filter_map(|x| {
            if let Ok(x) = x.parse::<i32>() {
                Some(x)
            } else {
                None
            }
        })
        .collect();

    nos.sort();
    //for i in 0..nos.len() {
    //    if let Some((j, _)) = nos.iter().enumerate().min_by_key(|x| x.1) {
    //        nos.swap(i, j);
    //    }
    //}

    //let no: Vec<_> = nos
    //    .iter()
    //    .filter_map(|x| {
    //        let sub = 2020 - x;
    //        if bs(&nos, 2020 - x) {
    //            if x > &sub {
    //                Some(*x)
    //            } else {
    //                Some(sub)
    //            }
    //        } else {
    //            None
    //        }
    //    })
    //    .collect();

    //for i in 0..no.len() {
    //    println!("{} {}", no[i], 2020 - no[i]);
    //    println!("{}", (no[i] * (2020 - no[i])));
    //}

    for i in 0..nos.len() {
        for j in 0..nos.len() {
            if i != j {
                if (nos[i] + nos[j]) <= 2020 {
                    let sub = 2020 - (nos[i] + nos[j]);
                    if nos[i] == 823 && nos[j] == 639 {
                        println!("{} {} {} {}", nos[i], nos[j], sub, bs(&nos, sub));
                    }
                    if bs(&nos, sub) {
                        println!("{}", nos[i] * nos[j] * sub);
                    }
                }
            }
        }
    }

    println!("{:?} {}", nos, bs(&nos, 558));
}

fn bs(nos: &Vec<i32>, no: i32) -> bool {
    let mut high = nos.len() - 1;
    let mut low = 0;
    while low <= high {
        let mid = (low + high) / 2;

        if nos[mid] == no {
            return true;
        } else if nos[mid] < no {
            low = mid + 1;
        } else {
            if mid <= 0 {
                //high = 0;
                return false;
            } else {
                high = mid - 1;
            }
        }
    }

    false
}

//use std::fs;
//
//fn main() {
//    let filename = "d1input.txt";
//
//    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
//    let mut numbers: Vec<_> = contents
//        .split('\n')
//        .filter_map(|n| {
//            if let Ok(x) = n.parse::<i32>() {
//                Some(x)
//            } else {
//                None
//            }
//        })
//        .collect();
//
//    for i in 0..numbers.len() {
//        if let Some((j, _)) = numbers.iter().enumerate().min_by_key(|x| x.1) {
//            numbers.swap(i, j);
//        }
//    }
//
//    for i in 0..numbers.len() {
//        //println!("{}", numbers[i]);
//        let s = 2020 - numbers[i];
//        if let Some(x) = bs(&numbers, s) {
//            println!("Some: {}, {} at index: {}", numbers[i], s, x);
//        } else {
//            println!("{}, {} at index: {}", numbers[i], s, -1);
//        }
//    }
//
//    //let mut ids: Vec<_>;
//    let idxs: Vec<_> = numbers
//        .iter()
//        .filter_map(|n| bs(&numbers, 2020 - n))
//        //.map(|n| println!("{}", n));
//        .collect();
//
//    let mut mul: i32 = 1;
//    for idx in idxs {
//        mul *= &numbers[idx];
//        println!("{}", numbers[idx]);
//    }
//
//    println!("{}", mul);
//}
//
//fn bs(numbers: &Vec<i32>, target: i32) -> Option<usize> {
//    let mut low: i32 = 0;
//    let mut high: i32 = numbers.len() as i32 - 1;
//    while low <= high {
//        let mid = ((high - low) / 2) + low;
//        let mid_index = mid as usize;
//        let val = numbers[mid_index];
//
//        if val == target {
//            println!("{}", numbers[mid_index]);
//            return Some(mid_index);
//        } else if val < target {
//            low = mid + 1;
//        } else {
//            high = mid - 1;
//        }
//    }
//    None
//}

//use itertools::Itertools;
//use std::io::{self, Read};
//fn main() {
//    let mut input = String::new();
//    io::stdin().read_to_string(&mut input).unwrap();
//    let nums: Vec<isize> = input.lines().map(|x| x.parse().unwrap()).collect();
//    println!("Part 1: {}", n_combo(&nums, 2).unwrap());
//    println!("Part 2: {}", n_combo(&nums, 3).unwrap());
//}
//fn n_combo(nums: &Vec<isize>, n: usize) -> Option<isize> {
//    for n in nums.iter().combinations(n) {
//        if n.iter().fold(0, |acc, &&n| acc + n) == 2020 {
//            return Some(n.iter().fold(1, |acc, &&n| acc * n));
//        }
//    }
//    None
//}
