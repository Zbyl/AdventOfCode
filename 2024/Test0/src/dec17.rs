use std::collections::HashSet;
use itertools::Itertools;
use regex::Regex;
use crate::helpers::read_lines;


#[derive(Debug, Clone)]
struct Input {
    a: i64,
    b: i64,
    c: i64,
    program: Vec<i64>,
}

fn parse_input(lines: &Vec<String>) -> crate::helpers::Result<Input> {
    let register_regex = Regex::new(r"^Register (?:A|B|C): (?<value>\d+)$").unwrap();
    let program_regex = Regex::new(r"^Program: (?<instructions>[0-9,]+)$").unwrap();
    let line0 = lines.get(0).unwrap();
    let line1 = lines.get(1).unwrap();
    let line2 = lines.get(2).unwrap();
    let line3 = lines.get(3).unwrap();
    let line4 = lines.get(4).unwrap();

    if !line3.is_empty() {
        return Err(format!("Line idx={} (zero-based) should be empty, but is: {}", 3, line3).into());
    }

    let register_a = register_regex.captures(line0).ok_or(format!("Line idx={} (zero-based) '{}' does not match the register regex.", 0, line0))?;
    let register_b = register_regex.captures(line1).ok_or(format!("Line idx={} (zero-based) '{}' does not match the register regex.", 1, line1))?;
    let register_c = register_regex.captures(line2).ok_or(format!("Line idx={} (zero-based) '{}' does not match the register regex.", 2, line2))?;
    let program = program_regex.captures(line4).ok_or(format!("Line idx={} (zero-based) '{}' does not match the program regex.", 4, line4))?;


    let input = Input {
        a: register_a["value"].parse::<i64>().unwrap(),
        b: register_b["value"].parse::<i64>().unwrap(),
        c: register_c["value"].parse::<i64>().unwrap(),
        program: program["instructions"].split(',').map(|i| i.parse::<i64>().unwrap()).collect_vec(),
    };

    Ok(input)
}

fn decode_combo_value(program: &Input, a: i64, b: i64, c: i64, val: i64) -> i64 {
    match val {
        0 | 1 | 2 | 3 => val,
        4 => a,
        5 => b,
        6 => c,
        _ => panic!("Combo value out of range: {}", val),
    }
}

fn decode_combo_sym(val: i64) -> String {
    match val {
        0 | 1 | 2 | 3 => val.to_string(),
        4 => "a".to_string(),
        5 => "b".to_string(),
        6 => "c".to_string(),
        _ => panic!("Combo value out of range: {}", val),
    }
}

fn decode(program: &Input) -> () {
    let mut pc: i64 = 0;

    loop {
        if pc >= program.program.len() as i64 {
            break;
        }

        let inst = program.program[pc as usize];
        let op = program.program[(pc + 1) as usize];
        pc += 2;

        match inst {
            0 => { println!("lab{}: a = a / (1 << {});", pc-2, decode_combo_sym(op)); }, // adv
            1 => { println!("lab{}: b = b ^ {};", pc-2, op); }, // bxl
            2 => { println!("lab{}: b = {} % 8;", pc-2, decode_combo_sym(op)); }, // bst
            3 => { println!("lab{}: if (a) {{ goto lab{}; }}", pc-2, op); }, // jnz
            4 => { println!("lab{}: b = b ^ c;", pc-2); }, // bxc
            5 => { println!("lab{}: print({} % 8);", pc-2, decode_combo_sym(op)); }, // out
            6 => { println!("lab{}: b = a / (1 << {});", pc-2, decode_combo_sym(op)); }, // bdv
            7 => { println!("lab{}: c = a / (1 << {});", pc-2, decode_combo_sym(op)); }, // cdv
            _ => panic!("Instruction value out of range: {}", inst),
        }
    }
}

fn compute(program: &Input) -> Vec<i64> {
    compute_ex(program, program.a, program.b, program.c)
}

fn compute_ex(program: &Input, pa: i64, pb: i64, pc: i64) -> Vec<i64> {
    let mut pc: i64 = 0;
    let mut result = Vec::<i64>::new();
    let mut a = pa;
    let mut b = pb;
    let mut c = pc;
    loop {
        if pc >= program.program.len() as i64 {
            break;
        }

        let inst = program.program[pc as usize];
        let op = program.program[(pc + 1) as usize];
        pc += 2;

        match inst {
            0 => { a = a / (1 << decode_combo_value(&program, a, b, c, op)); }, // adv
            1 => { b = b ^ op; }, // bxl
            2 => { b = decode_combo_value(&program, a, b, c, op) % 8; }, // bst
            3 => { if a != 0 { pc = op; } }, // jnz
            4 => { b = b ^ c }, // bxc
            5 => { result.push(decode_combo_value(&program, a, b, c, op) % 8) }, // out
            6 => { b = a / (1 << decode_combo_value(&program, a, b, c, op)); }, // bdv
            7 => { c = a / (1 << decode_combo_value(&program, a, b, c, op)); }, // cdv
            _ => panic!("Instruction value out of range: {}", inst),
        }
    }

    result
}

fn to_digits(num: i64) -> Vec<i64> {
    let mut nums = Vec::<i64>::new();
    let mut ca = num;
    loop {
        let l = ca % 8;
        nums.push(l);
        ca = ca / 8;
        if ca == 0 { break; }
    }
    nums
}

fn from_digits(nums: &Vec<i64>) -> i64 {
    let mut num = 0;
    for l in nums.iter().rev() {
        num = num * 8 + l;
    }
    num
}

fn multi_compute(program: &Input, last_digs: &Vec<i64>) -> i64 {
    let b = program.b;
    let c = program.c;
    let mut next_candidates: HashSet<i64> = HashSet::new();
    for a in 0..300000 {
        if a % 100000 == 0 {
            //println!("{:?}", a);
        }

        let nums = to_digits(a);
        let mut effnums = nums.clone();
        effnums.extend(last_digs);
        let effective_a = from_digits(&effnums);

        let result = compute_ex(program, effective_a, b, c);

        let num_match = last_digs.len() + 1;
        let last_res = &result[result.len() - num_match..];
        let last_prg = &program.program[program.program.len() - num_match..];
        if last_res == last_prg {
            println!("{} | {}: {:?} {:?} -> {:?}", a, effective_a, nums, last_digs, result);
            next_candidates.insert(*nums.last().unwrap());
        }
        if result == program.program {
            println!("Success: {:?}", effective_a);
            return a;
        }
    }
    println!("Candidates: {:?}", next_candidates);
    return 0;
}

fn multi_compute_b(program: &Input, expected: &Vec<i64>) -> i64 {
    let b = program.b;
    let c = program.c;
    let mut last_digs: Vec<i64> = Vec::new();
    loop {
        for a in 1.. {
            let nums = to_digits(a);
            let mut effnums = nums.clone();
            effnums.extend(&last_digs);
            let effective_a = from_digits(&effnums);
            let result = compute_ex(program, effective_a, b, c);
            println!("{} | {}: {:?} {:?} = {:?} -> {:?}", a, effective_a, nums, last_digs, effnums, result);

            let num_match = last_digs.len() + 1;
            let last_res = &result[result.len() - num_match..];
            let last_prg = &program.program[program.program.len() - num_match..];
            if last_res == last_prg {
                last_digs.push(*nums.last().unwrap());
                continue;
            }
            if result == program.program {
                return a;
            }
        }
    }
}

fn prog(program: &Input, expected: &Vec<i64>) -> bool{
    let mut a = program.a;
    let mut b = program.b;
    let mut c = program.c;
    let mut idx = 0;

    let mut print = |op| -> bool {
        if idx >= expected.len() { return false; }
        if op != expected[idx] { return false; }
        idx += 1;
        true
    };

    loop {
        b = a % 8;
        b = b ^ 7;
        c = a / (1 << b);
        b = b ^ c;
        b = b ^ 4;
        let v = b % 8;
        if (!print(v)) {
            //println!("Offending idx={} have v={} expected {:?}", idx, v, expected.get(idx));
            return false;
        }
        a = a / (1 << 3);
        if a == 0 { break; }
    }

    if (idx != expected.len()) {
        //println!("idx={} but input has size {}", idx, expected.len());
        return false;
    }
    true
}

fn prog0(program: &Input, expected: &Vec<i64>) -> bool{
    let mut a = program.a;
    let mut b = program.b;
    let mut c = program.c;
    let mut idx = 0;

    let mut print = |op| -> bool {
        if idx >= expected.len() {
            return false;
        }
        if op != expected[idx] {
            return false;
        }
        idx += 1;
        true
    };

    loop {
        a = a / (1 << 1);
        let v = a % 8;
        if (!print(v)) {
            println!("Offending idx={} have v={} expected {:?}", idx, v, expected.get(idx));
            return false;
        }
        if a == 0 { break; }
    }

    if (idx != expected.len()) {
        println!("idx={} but input has size {}", idx, expected.len());
        return false;
    }
    true
}

fn multi_compute_prog(program: &Input) -> i64 {
    //let expected = vec![2,1,0,4,6,2,4,2,0];
    let expected = vec![2,4,1,7,7,5,4,1,1,4,5,5,0,3,3,0];
    let mut progo = program.clone();
    for a in 15013000000i64.. {
        if a % 1000000 == 0 {
            println!("{:?}", a);
        }
        progo.a = a;
        let result = prog(&progo, &expected);
        if result { return a; }
    }
    unreachable!()
}

fn multi_compute_prog_try(program: &Input) -> i64 {
    //let expected = vec![2,1,0,4,6,2,4,2,0];
    let expected = vec![2, 4]; //,4,1,7]; //,7,5,4,1,1,4,5,5,0,3,3,0];
    let mut progo = program.clone();
    for a in 4142000000i64.. {
        if a % 1000000 == 0 {
            println!("{:?}", a);
        }
        progo.a = a;
        let result = prog(&progo, &expected);
        if result { return a; }
    }
    unreachable!()
}

#[allow(dead_code)]
pub(crate) fn dec17() {
    let lines = read_lines("dec17.in.txt").expect("Could not load input.");
    let mut input = parse_input(&lines).unwrap();
    let result = compute(&mut input);
    let sres = result.iter().map(|i| i.to_string()).join(",");
    println!("{:?}", sres);
}

#[allow(dead_code)]
pub(crate) fn dec17_2() {
    let lines = read_lines("dec17.in.txt").expect("Could not load input.");
    let input = parse_input(&lines).unwrap();
    decode(&input);
    println!("{:?}",compute(&input));
    //let result = prog0(&input, &vec![4,6,3,5,6,3,5,2,1,0]);
    //let result = prog(&input, &vec![2,1,0,4,6,2,4,2,0]);
    //println!("{:?}",input);
    //println!("{:?}",compute(&mut input.clone()));
    // Last: 3
    //       0 or 7
    //            0: 7, 0, 3 -> 2 or 4
    //               2: 2, 7, 0, 3 -> 1 or 3 or 4
    //                  1: 1, 2, 7, 0, 3 -> 2 or 5
    //                     2: none
    //                     5: none
    //                  3: 2, 3, 2, 7, 0, 3 -> none
    //                  4: none
    //               4: 1, 4, 7, 0, 3 -> 0 or 5
    //                  0: 3, 0, 1, 4, 7, 0, 3 -> 2 or 3 or 5
    //                     2: 1, 2, 3, 0, 1, 4, 7, 0, 3 -> none
    //                     3: 3, 3, 0, 1, 4, 7, 0, 3 -> 1 or 5
    //                         success a=109685330781408 res = 107744, [0, 4, 3, 2, 2, 3] [3, 1, 3, 3, 0, 1, 4, 7, 0, 3]
    //            7: 1, 4, 7, 7, 3 -> 0 or 5
    //                                     5: none
    //                                     0: 3, 0, 1, 4, 7, 7, 3 -> 2 or 5 or 3
    //                                                                  2: 1, 3, 0, 1, 4, 7, 7, 3 -> none
    //                                                                  5: 3, 1, 5, 3, 0, 1, 4, 7, 7, 3 -> success a=140471689913568 res=107744 not right!
    //                                                                  3: 3, 3, 0, 1, 4, 7, 7, 3 -> 1 or 5
    let result = multi_compute(&input, &vec![3, 1, 3, 3, 0, 1, 4, 7, 0, 3]);
    //let result = multi_compute_prog(&input);
    //let result = multi_compute_b(&input, &vec![2,4,1,7,7,5,4,1,1,4,5,5,0,3,3,0]);
    println!("{:?}", result);
}

