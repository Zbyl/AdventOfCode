use std::cmp;
use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;
use crate::helpers::{read_lines, Vec2};

type MapType = HashMap<char, HashMap<char, Vec<String>>>;

fn compute_map(forbidden_point: Vec2, char_pos: &HashMap<char, Vec2>) -> MapType {
    let mut out_map: MapType = HashMap::new();
    for &c0 in char_pos.keys() {
        for &c1 in char_pos.keys() {
            let p0 = char_pos.get(&c0).unwrap();
            let p1 = char_pos.get(&c1).unwrap();
            let mut res_x = (if p1.x > p0.x { ">" } else { "<" }).repeat((p1.x - p0.x).abs() as usize);
            let mut res_y = (if p1.y > p0.y { "v" } else { "^" }).repeat((p1.y - p0.y).abs() as usize);
            let mut res0 = Some(res_x.clone() + &res_y);
            let mut res1 = Some(res_y + &res_x);
            if (p0.y == forbidden_point.y) && (p1.x == forbidden_point.x) {
                res0 = None;
            }
            if (p0.x == forbidden_point.x) && (p1.y == forbidden_point.y) {
                res1 = None;
            }

            if !out_map.contains_key(&c0) {
                out_map.insert(c0, HashMap::new());
            }
            let mut res: Vec<String> = Vec::new();
            if res0.is_some() { res.push(res0.clone().unwrap()); }
            if res1.is_some() && (res1 != res0) { res.push(res1.unwrap()); }
            out_map.get_mut(&c0).unwrap().insert(c1, res);
        }
    }

    out_map
}

fn compute_maps() -> (MapType, MapType) {
    /*
    let bot_map = hashmap! {
        'A' => hashmap! {
            'A' => "".to_string(),
            '^' => "<".to_string(),
            'v' => "<v".to_string(),
            '<' => "v<<".to_string(),
            '>' => "v".to_string(),
        },
        '^' => hashmap! {
            'A' => ">".to_string(),
            '^' => "".to_string(),
            'v' => "v".to_string(),
            '<' => "v<".to_string(),
            '>' => "v>".to_string(),
        },
        'v' => hashmap! {
            'A' => "^>".to_string(),
            '^' => "^".to_string(),
            'v' => "".to_string(),
            '<' => "<".to_string(),
            '>' => ">".to_string(),
        },
        '>' => hashmap! {
            'A' => "^".to_string(),
            '^' => "^<".to_string(),
            'v' => "<".to_string(),
            '<' => "<<".to_string(),
            '>' => "".to_string(),
        },
        '<' => hashmap! {
            'A' => ">>^".to_string(),
            '^' => ">^".to_string(),
            'v' => ">".to_string(),
            '<' => "".to_string(),
            '>' => ">>".to_string(),
        },
    };
    */

    let bot_pos: HashMap<char, Vec2> = hashmap! {
        '^' => Vec2::new(1, 0),
        'A' => Vec2::new(2, 0),

        '<' => Vec2::new(0, 1),
        'v' => Vec2::new(1, 1),
        '>' => Vec2::new(2, 1),
    };

    let door_pos: HashMap<char, Vec2> = hashmap! {
        '7' => Vec2::new(0, 0),
        '8' => Vec2::new(1, 0),
        '9' => Vec2::new(2, 0),

        '4' => Vec2::new(0, 1),
        '5' => Vec2::new(1, 1),
        '6' => Vec2::new(2, 1),

        '1' => Vec2::new(0, 2),
        '2' => Vec2::new(1, 2),
        '3' => Vec2::new(2, 2),

        '0' => Vec2::new(1, 3),
        'A' => Vec2::new(2, 3),
    };

    let bot_map = compute_map(Vec2::new(0, 0), &bot_pos);
    let door_map = compute_map(Vec2::new(0, 3), &door_pos);
    (bot_map, door_map)
}

fn map_str_bad(line: &String, map: &MapType) -> String {
    let mut res = String::new();
    let mut cur_c = 'A';
    for c in line.chars() {
        let mut best = map[&cur_c][&c][0].as_str();
        if !res.is_empty() {
            let last = res.chars().last().unwrap();
            for candidate in map[&cur_c][&c].iter() {
                if candidate == "" {
                    best = candidate.as_str();
                    break;
                }
                if candidate.chars().last().unwrap() == last {
                    best = candidate.as_str();
                    break;
                }
            }
        }
        res += best;
        res += "A";
        cur_c = c;
    }
    res
}

fn decompose_str(s: &str) -> Option<(char, &str)> {
    let mut chars = s.chars();
    chars.next().map(|first| (first, chars.as_str()))
}

fn map_str_x(cur_c: char, line: &str, map: &MapType, cache: &mut HashMap<(char, String), String>) -> String {
    let line_s = line.to_string();
    if line.is_empty() { return line_s; }

    if let Some(res) = cache.get(&(cur_c, line_s.clone())) {
        return res.clone();
    }

    let (c, rest) = decompose_str(line).unwrap();

    let candidates = &map[&cur_c][&c];
    let mut best: Option<String> = None;
    for candidate in candidates.iter() {
        let new_best = candidate.to_owned() + "A" + &map_str_x(c, rest, map, cache);
        match best {
            None => best = Some(new_best),
            Some(ref s) => if new_best.len() < s.len() { best = Some(new_best); }
        }
    }

    let best = best.unwrap();
    cache.insert((cur_c, line_s), best.clone());
    best
}

fn map_str(line: &str, map: &MapType) -> String {
    let mut cache: HashMap<(char, String), String> = HashMap::new();
    map_str_x('A', line, map, &mut cache)
}

fn compute_one(line: &String, bot_map: &MapType, door_map: &MapType) -> i64 {
    let code = line.strip_suffix("A").unwrap().parse::<i64>().unwrap();

    let bot0 = map_str(line, door_map);
    let bot1 = map_str(&bot0, bot_map);
    //let bot2 = map_str(&bot1, bot_map);
    let man = map_str(&bot1, bot_map);

    println!("bot0: {bot0}");
    println!("bot1: {bot1}");
    //println!("bot2: {bot2}");
    println!("man:  {man}");

    let res = man.len() as i64 * code;
    println!("res:  {} * {code} = {res}", man.len());
    res
}

/*
fn all_possible_expansions(cur_c: char, line: &str, maps: &[MapType]) -> Vec<String> {
    if maps.is_empty() { return vec![line.to_string()]; }
    if line.is_empty() { return vec![line.to_string()]; }

    let (c, rest) = decompose_str(line).unwrap();
    let [map, other_maps @ ..] = maps;

    let candidates = &map[&cur_c][&c];
    let mut result = Vec::new();
    for candidate in candidates.iter() {
        let cand_options = all_possible_expansions(cur_c, rest, map, cache);
        let options = all_possible_expansions(c, rest, map, cache);
        candidate.to_owned() + "A" + &
    }
    best.unwrap()
}
*/

fn compute(lines: &Vec<String>, bot_map: &MapType, door_map: &MapType) -> i64 {
    let mut result = 0;
    for line in lines {
        let res = compute_one(line, &bot_map, &door_map);
        result += res;
    }
    result
}


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct BotManState {
    output: String,
    bot0: Vec2,
    bot1: Vec2,
    man: Vec2,
}

fn simulate_basic(pos: Vec2, input: char, pos_to_out: &HashMap<Vec2, char>) -> Option<(Vec2, Option<char>)> {
    if input == 'A' {
        let out = pos_to_out[&pos];
        return Some((pos, Some(out)));
    }

    let dir = match input {
        '<' => Vec2::new(-1, 0),
        '>' => Vec2::new(1, 0),
        '^' => Vec2::new(0, -1),
        'v' => Vec2::new(0, 1),
        _ => unreachable!(),
    };

    let next_pos = pos + dir;
    if !pos_to_out.contains_key(&next_pos) {
        return None;
    }

    Some((next_pos, None))
}

type State = Vec<(Vec2, bool)>;  // (Current pos, is door) (x N), output

fn simulate(states: &State, input: char) -> Option<(State, Option<char>)> {
    let door_pos_to_out = hashmap! {
        Vec2::new(0, 0) => '7',
        Vec2::new(1, 0) => '8',
        Vec2::new(2, 0) => '9',

        Vec2::new(0, 1) => '4',
        Vec2::new(1, 1) => '5',
        Vec2::new(2, 1) => '6',

        Vec2::new(0, 2) => '1',
        Vec2::new(1, 2) => '2',
        Vec2::new(2, 2) => '3',

        Vec2::new(1, 3) => '0',
        Vec2::new(2, 3) => 'A',
    };

    let bot_pos_to_out = hashmap! {
        Vec2::new(1, 0) => '^',
        Vec2::new(2, 0) => 'A',

        Vec2::new(0, 1) => '<',
        Vec2::new(1, 1) => 'v',
        Vec2::new(2, 1) => '>',
    };

    let mut new_states = states.clone();
    let mut cur_input = input;
    for (idx, (pos, is_door)) in states.iter().enumerate() {
        let basic_res = simulate_basic(*pos, cur_input, if *is_door { &door_pos_to_out } else { &bot_pos_to_out });
        if basic_res.is_none() {
            return None;
        }
        let (new_pos, new_out) = basic_res.unwrap();
        new_states[idx] = (new_pos, *is_door);
        if new_out.is_none() {
            return Some((new_states, None));
        }

        cur_input = new_out.unwrap();
    }

    Some((new_states, Some(cur_input)))
}

fn zero_state() -> State {
    vec![(Vec2::new(2, 0), false), (Vec2::new(2, 0), false), (Vec2::new(2, 3), true)]
}

fn zero_state_max(dir_keypads: i32) -> State {
    let mut result = Vec::new();
    for i in 0..dir_keypads {
        result.push((Vec2::new(2, 0), false));
    }
    result.push((Vec2::new(2, 3), true));
    result
}

fn simulate_input(input: &str) -> Option<String> {
    let mut states = zero_state();
    let mut output = Vec::new();
    for c in input.chars() {
        let new_state = simulate(&states, c);
        //println!("{} -> {:?}", c, new_state);
        if new_state.is_none() { return None; }
        let (new_states, out) = new_state.unwrap();
        states = new_states;
        if out.is_some() { output.push(out.unwrap()); }
    }
    Some(output.into_iter().collect())
}

type OutState = (String, State, String); // Path, State, Output.

fn simulate_bfs(desired_output: &str, zero: &State) -> Option<String> {
    let mut queue: VecDeque<OutState> = VecDeque::new();
    queue.push_back(("".to_string(), zero.clone(), "".to_string()));
    let mut visited: HashSet<(State, String)> = HashSet::new();
    visited.insert((zero.clone(), "".to_string()));

    let mut found_len = 0;

    loop {
        if queue.is_empty() { return None; }
        let (path, states, output) = queue.pop_front().unwrap();
        //println!("path: {path:?} output: {output:?}");
        if output == desired_output { return Some(path); }
        if output.len() < found_len { continue; }

        for c in ['<', '>', '^', 'v', 'A'] {
            let new_state = simulate(&states, c);
            if new_state.is_none() { continue; }
            let (new_states, out) = new_state.unwrap();
            let new_output = if let Some(out) = out { output.clone() + &out.to_string() } else { output.clone() };

            if !desired_output.starts_with(new_output.as_str()) {
                continue;
            }
            found_len = cmp::max(found_len, new_output.len());

            let vis_key = (new_states.clone(), new_output.clone());
            if visited.contains(&vis_key) { continue; }
            queue.push_back((path.clone() + &c.to_string(), new_states, new_output));
            visited.insert(vis_key);
        }
    }
}

fn compute_one2(line: &String, zero: &State) -> i64 {
    let code = line.strip_suffix("A").unwrap().parse::<i64>().unwrap();

    let result = simulate_bfs(&line, zero).expect(format!("Could not produce desired code: {}", line).as_str());

    let res = result.len() as i64 * code;
    println!("res:  {} * {code} = {res}", result.len());
    res
}

fn compute2(lines: &Vec<String>, zero: &State) -> i64 {
    let mut result = 0;
    for line in lines {
        let res = compute_one2(line, zero);
        result += res;
    }
    result
}

#[allow(dead_code)]
pub(crate) fn dec21() {
    let lines = read_lines("dec21.in.txt").expect("Could not load input.");

    //let result = simulate_bfs("029A");
    //println!("{:?}", result);

    let result = compute2(&lines, &zero_state_max(10));
    println!("{:?}", result);
    /*
    for line in lines {
        let out = simulate_input(&line);
        println!("{:?}", out);
    }
    */
}

// 135260 - too high.
// 818240 - too low for 26
