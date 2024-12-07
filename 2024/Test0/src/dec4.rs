
use crate::helpers::{read_matrix, Matrix, Vec2};

fn match_word(matrix: &Matrix, word: &str, x: i32, y: i32, dx: i32, dy: i32) -> bool {
    for (idx, wc) in word.chars().enumerate() {
        let mc = matrix.get(Vec2::new(x + (idx as i32) * dx, y + (idx as i32) * dy)).unwrap_or('?');
        if mc != wc {
            return false
        }
    }
    true
}

fn match_dir(matrix: &Matrix, word: &str, dx: i32, dy: i32) -> i32 {
    let mut res = 0;
    for x in 0..matrix.width {
        for y in 0..matrix.height {
            if match_word(matrix, word, x as i32, y as i32, dx, dy) {
                res += 1;
            }
        }
    }
    res
}

fn match_matrix(matrix: &Matrix, word: &str) -> i32 {
    0
        + match_dir(matrix, word, 1, 0)
        + match_dir(matrix, word, -1, 0)
        + match_dir(matrix, word, 0, 1)
        + match_dir(matrix, word, 0, -1)
        + match_dir(matrix, word, 1, 1)
        + match_dir(matrix, word, -1, -1)
        + match_dir(matrix, word, 1, -1)
        + match_dir(matrix, word, -1, 1)
}

fn match_x(matrix: &Matrix, x: i32, y: i32) -> bool {
    let diag_tl = match_word(matrix, "MAS", x - 1, y - 1, 1, 1);
    let diag_br = match_word(matrix, "MAS", x + 1, y + 1, -1, -1);
    let diag_tr = match_word(matrix, "MAS", x + 1, y - 1, -1, 1);
    let diag_bl = match_word(matrix, "MAS", x - 1, y + 1, 1, -1);
    (diag_tl || diag_br) && (diag_tr || diag_bl)
}

fn match_matrix_2(matrix: &Matrix) -> i32 {
    let mut res = 0;
    for x in 0..matrix.width {
        for y in 0..matrix.height {
            if match_x(matrix, x as i32, y as i32) {
                res += 1;
            }
        }
    }
    res
}

#[allow(dead_code)]
pub(crate) fn dec4() {
    let matrix = read_matrix("dec4.in.txt").expect("Could not load input.");
    //println!("{:?}", matrix);
    let res = match_matrix(&matrix, "XMAS");
    println!("{:?}", res);
}

#[allow(dead_code)]
pub(crate) fn dec4_2() {
    let matrix = read_matrix("dec4.in.txt").expect("Could not load input.");
    //println!("{:?}", matrix);
    let res = match_matrix_2(&matrix);
    println!("{:?}", res);
}
