use std::thread;
use std::time;
use regex::Regex;
use crate::helpers::{read_lines, Vec2};

#[derive(Debug, Clone, Copy)]
struct Input {
    p: Vec2<i64>,
    v: Vec2<i64>,
}

fn parse_input(lines: &Vec<String>) -> crate::helpers::Result<Vec<Input>> {
    let mut inputs: Vec<Input> = Vec::new();
    //return Err(format!("Line idx={} (zero-based) {} does not match the rule regex.", 0, 1).into());

    let in_regex = Regex::new(r"^p=(?<px>-?\d+),(?<py>-?\d+) v=(?<vx>-?\d+),(?<vy>-?\d+)$").unwrap();
    for (idx, line) in lines.iter().enumerate() {
        let cap = in_regex.captures(line).ok_or(format!("Line idx={} (zero-based) '{}' does not match the regex.", idx, line))?;

        let input = Input {
            p: Vec2::<i64>::new(cap["px"].parse::<i64>().unwrap(), cap["py"].parse::<i64>().unwrap()),
            v: Vec2::<i64>::new(cap["vx"].parse::<i64>().unwrap(), cap["vy"].parse::<i64>().unwrap()),
        };
        inputs.push(input);
    }

    Ok(inputs)
}

fn compute_stuff(inputs: &mut Vec<Input>, size: Vec2<i64>, time: i64) -> i64 {
    let mut ul = 0;
    let mut ur = 0;
    let mut bl = 0;
    let mut br = 0;

    let mid = size / 2;
    for input in inputs {
        input.p += input.v * time;
        input.p.x %= size.x;
        input.p.y %= size.y;
        if (input.p.x < 0) { input.p.x += size.x; }
        if (input.p.y < 0) { input.p.y += size.y; }

        if (input.p.x < mid.x) {
            if input.p.y < mid.y {
                ul += 1;
            }
            else if input.p.y > mid.y {
                bl += 1;
            }
        }
        else if (input.p.x > mid.x) {
            if input.p.y < mid.y {
                ur += 1;
            }
            else if input.p.y > mid.y {
                br += 1;
            }
        }
    }

    ul * ur * bl * br
}

fn compute_stuff2(inputs: &Vec<Input>, size: Vec2<i64>) -> i64 {
    /*
    let mut reference = HashSet::new();
    for i in 0..size.x {
        reference.insert(Vec2::new(i, (size.y - 1 - i).abs()));
    }
    */
    for time in 8280..8281 {
        //let mut pic = reference.clone();
        let mut pic = vec![0; (size.x * size.y) as usize];
        for input in inputs.iter() {
            let mut pos = input.p + input.v * time;
            pos.x %= size.x;
            pos.y %= size.y;
            if (pos.x < 0) { pos.x += size.x; }
            if (pos.y < 0) { pos.y += size.y; }
            //pic.remove(&pos);
            pic[(pos.y * size.x + pos.x) as usize] += 1;
        }
        println!();
        println!();
        println!();
        println!();
        println!("{}", time);
        println!();
        println!();
        for y in 0..size.y {
            for x in 0..size.x {
                let idx = y * size.x + x;
                let val = pic[idx as usize];
                print!("{}", if val > 0 { (('0' as u8) + val) as char } else { ' ' });
            }
            println!();
        }
        let ten_millis = time::Duration::from_millis(50);
        thread::sleep(ten_millis);
    }
    unreachable!();
}

#[allow(dead_code)]
#[allow(unused_variables)]
pub(crate) fn dec14() {
    let lines = read_lines("dec14.in.txt").expect("Could not load input.");
    let mut inputs = parse_input(&lines).unwrap();
    let ex_size = Vec2::<i64>::new(11, 7);
    let in_size = Vec2::<i64>::new(101, 103);
    let result = compute_stuff(&mut inputs, in_size, 100);
    println!("{:?}", result);
}

#[allow(dead_code)]
#[allow(unused_variables)]
pub(crate) fn dec14_2() {
    let lines = read_lines("dec14.in.txt").expect("Could not load input.");
    let inputs = parse_input(&lines).unwrap();
    let ex_size = Vec2::<i64>::new(11, 7);
    let in_size = Vec2::<i64>::new(101, 103);
    let result = compute_stuff2(&inputs, in_size);
    println!("{:?}", result);
}
