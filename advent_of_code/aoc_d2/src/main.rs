//use std::fs;
//
//fn main() {
//    let filename = "d2input.txt";
//
//    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
//
//    //(i32, i32, &str, &str)
//
//    let mut t = Vec::new();
//    let _nos: Vec<i32> = contents
//        .split('\n')
//        .filter_map(|x| {
//            let p1: Vec<&str> = x.split(": ").collect();
//            let p2: Vec<&str> = p1[0].split(" ").collect();
//            let p3: Vec<&str> = p2[0].split("-").collect();
//            let n1 = match p3[0].parse::<i32>() {
//                Ok(x) => x,
//                Err(_) => return None,
//            };
//
//            let n2 = match p3[1].parse::<i32>() {
//                Ok(x) => x,
//                Err(_) => return None,
//            };
//
//            t.push((n1, n2, p2[1], p1[1]));
//            Some(n1)
//        })
//        .collect();
//
//    let t1: Vec<_> = t
//        .iter()
//        .filter(|x| {
//            let cnt = x.3.matches(x.2).count() as i32;
//            if cnt >= x.0 && cnt <= x.1 {
//                println!("{}, {}, {}, {}, {}, true", x.0, x.1, x.2, x.3, cnt);
//                true
//            } else {
//                //println!("{}, {}, {}, {}, {}, false", x.0, x.1, x.2, x.3, cnt);
//                false
//            }
//        })
//        .collect();
//
//    println!("{}", t1.len());
//}

use std::{env::args, fs::read_to_string};

fn split_at_once(input: &str, sep: char) -> Option<(&str, &str)> {
    let mut s = input.splitn(2, sep);
    s.next().and_then(|n| s.next().map(|v| (n, v)))
}

fn parse_input(input: &str) -> impl Iterator<Item = (usize, usize, char, &str)> {
    input
        .lines()
        .filter_map(|l| {
            split_at_once(l, '-').and_then(|(min, rest)| {
                split_at_once(rest, ' ')
                    .map(|(max, rest)| (min, max, rest))
                    .and_then(|(min, max, rest)| {
                        split_at_once(rest, ':').map(|(c, rest)| (min, max, c, rest.trim()))
                    })
            })
        })
        .filter_map(|(min, max, c, s)| {
            Some((
                min.parse::<usize>().ok()?,
                max.parse::<usize>().ok()?,
                c.chars().next()?,
                s,
            ))
        })
}

fn part1(input: &str) -> usize {
    parse_input(input)
        .map(|(min, max, c, s)| (min, max, s.chars().filter(|&ch| ch == c).count()))
        .filter(|(min, max, count)| min <= count && count <= max)
        .count()
}

fn part2(input: &str) -> usize {
    parse_input(input)
        .filter(|(min, max, c, s)| {
            let mut chars = s.chars();
            let c1 = chars.nth(min - 1);
            let c2 = chars.nth(max - min - 1);
            (c1 == Some(*c)) ^ (c2 == Some(*c))
        })
        .count()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_to_string(args().nth(1).ok_or("No input file")?)?;

    let len1 = part1(&input);
    let len2 = part2(&input);
    println!("part 1: {}", len1);
    println!("part 2: {}", len2);

    Ok(())
}
