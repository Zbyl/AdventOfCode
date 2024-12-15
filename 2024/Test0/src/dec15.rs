use crate::dec6::{make_maze, Maze};
#[allow(unused_imports)]
use crate::helpers::{print_maze, read_lines, read_matrix_from_lines, Matrix, Vec2};

fn separate(lines: Vec<String>) -> (Vec<String>, Vec<String>) {
    let mut s0: Vec<String> = Vec::new();
    let mut s1: Vec<String> = Vec::new();
    let mut first = true;
    for line in lines {
        if line.is_empty() {
            first = false;
            continue;
        }
        if first {
            s0.push(line);
        } else {
            s1.push(line);
        }
    }

    (s0, s1)
}

fn compute_result(matrix: &Matrix) -> i64 {
    let mut result = 0;
    for y in 0..matrix.height {
        for x in 0..matrix.width {
            let pos = Vec2::new(x as i32, y as i32);
            let val = matrix.get(pos).unwrap();
            if val == 'O' {
                let coord = y * 100 + x;
                result += coord as i64;
            }
        }
    }
    result
}

fn compute_result2(matrix: &Matrix) -> i64 {
    let mut result = 0;
    for y in 0..matrix.height {
        for x in 0..matrix.width {
            let pos = Vec2::new(x as i32, y as i32);
            let val = matrix.get(pos).unwrap();
            if val == '[' {
                let coord = y * 100 + x;
                result += coord as i64;
            }
        }
    }
    result
}

fn process_commands(maze: &mut Maze, commands: &str) -> () {
    for c in commands.chars() {
        let dir = match c {
            '<' => Vec2::left(),
            '>' => Vec2::right(),
            '^' => Vec2::up(),
            'v' => Vec2::down(),
            _ => panic!("Unknown command: {}", c),
        };

        let mut tunnel_pos = maze.start;
        let mut tunnel_char;
        loop {
            let next_pos = tunnel_pos + dir;
            tunnel_char = maze.matrix.get(next_pos).expect(format!("The next pos is beyond the map: {:?}", next_pos).as_str());
            tunnel_pos = next_pos;
            if tunnel_char != 'O' {
                break;
            }
        }
        if tunnel_char == '#' {
            //println!("Command: {} ignored", c);
            continue;
        }
        maze.matrix.put(tunnel_pos, 'O');
        maze.matrix.put(maze.start, '.');
        maze.matrix.put(maze.start + dir, '@');
        maze.start += dir;

        //println!("Command: {}", c);
        //print_maze(maze, '@');
    }
}

fn can_tunnel(matrix: &Matrix, pos: Vec2, dir: Vec2) -> bool {
    let next_pos = pos + dir;
    let val = matrix.get(next_pos).expect(format!("The next pos is beyond the map: {:?}", next_pos).as_str());
    match val {
        '#' => return false,
        '.' => return true,
        '[' => if dir.x == 0 {
            // Vertical movement.
            return can_tunnel(matrix, next_pos, dir) && can_tunnel(matrix, next_pos + Vec2::right(), dir)
        } else {
            // Horizontal movement.
            return can_tunnel(matrix, next_pos, dir)
        }
        ']' => if dir.x == 0 {
            // Vertical movement.
            return can_tunnel(matrix, next_pos, dir) && can_tunnel(matrix, next_pos + Vec2::left(), dir)
        } else {
            // Horizontal movement.
            return can_tunnel(matrix, next_pos, dir)
        }
        _ => panic!("Unexpected value at {:?}: {}", next_pos, val),
    }
}

fn do_tunnel(matrix: &mut Matrix, pos: Vec2, dir: Vec2, replacement_char: char) {
    let next_pos = pos + dir;
    let val = matrix.get(pos).expect(format!("The next pos is beyond the map: {:?}", next_pos).as_str());
    let next_val = matrix.get(next_pos).expect(format!("The next pos is beyond the map: {:?}", next_pos).as_str());
    matrix.put(pos, replacement_char);
    match next_val {
        '#' => panic!("Hit wall at {:?}: {}", next_pos, next_val),
        '.' => matrix.put(next_pos, val),
        '[' => if dir.x == 0 {
            // Vertical movement.
            do_tunnel(matrix, next_pos, dir, val);
            do_tunnel(matrix, next_pos + Vec2::right(), dir, '.');
        } else {
            // Horizontal movement.
            do_tunnel(matrix, next_pos, dir, val);
        }
        ']' => if dir.x == 0 {
            // Vertical movement.
            do_tunnel(matrix, next_pos, dir, val);
            do_tunnel(matrix, next_pos + Vec2::left(), dir, '.');
        } else {
            // Horizontal movement.
            do_tunnel(matrix, next_pos, dir, val);
        }
        _ => panic!("Unexpected value at {:?}: {}", next_pos, val),
    }
}

fn process_commands2(maze: &mut Maze, commands: &str) -> () {
    for c in commands.chars() {
        let dir = match c {
            '<' => Vec2::left(),
            '>' => Vec2::right(),
            '^' => Vec2::up(),
            'v' => Vec2::down(),
            _ => panic!("Unknown command: {}", c),
        };

        if !can_tunnel(&maze.matrix, maze.start, dir) {
            //println!("Command: {} ignored", c);
            continue;
        }
        do_tunnel(&mut maze.matrix, maze.start, dir, '.');
        maze.start += dir;

        //println!("Command: {}", c);
        //print_maze(maze, '@');
    }
}

#[allow(dead_code)]
pub(crate) fn dec15() {
    let lines = read_lines("dec15.in.txt").expect("Could not load input.");
    let (lines0, lines1) = separate(lines);
    let matrix = read_matrix_from_lines(lines0).unwrap();
    let mut maze = make_maze(matrix, '@');
    let commands = lines1.join("").replace("\n", "");
    process_commands(&mut maze, &commands);
    let result = compute_result(&maze.matrix);
    println!("{:?}", result);
}


fn enlarge(matrix: Matrix) -> Matrix {
    let mut bigdata = Vec::new();
    for line in matrix.data {
        let newline = line.replace("#", "##").replace("O", "[]").replace(".", "..").replace("@", "@.");
        bigdata.push(newline);
    }
    Matrix { width: matrix.width * 2, height: matrix.height, data: bigdata }
}

#[allow(dead_code)]
pub(crate) fn dec15_2() {
    let lines = read_lines("dec15.in.txt").expect("Could not load input.");
    let (lines0, lines1) = separate(lines);
    let matrix = read_matrix_from_lines(lines0).unwrap();
    let bigmatrix = enlarge(matrix);
    let mut maze = make_maze(bigmatrix, '@');
    let commands = lines1.join("").replace("\n", "");
    process_commands2(&mut maze, &commands);
    let result = compute_result2(&maze.matrix);
    println!("{:?}", result);
}
