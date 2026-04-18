pub mod dstruct;

use dstruct::Matrix;

pub fn run_iteration(source: &Matrix<bool>, dest: &mut Matrix<bool>) {
    if source.width() != dest.width() || source.height() != dest.height() {
        panic!("Source and dest matricies must be the same size")
    }

    for y in 0..source.height() {
        for x in 0..source.width() {
            if *source.get(x, y).unwrap() == true {
                dest.set(x, y, check_live(x, y, source));
            } else {
                dest.set(x, y, check_dead(x, y, source));
            }
        }
    }
}

fn check_live(x: usize, y: usize, matrix: &Matrix<bool>) -> bool {
    let num_neighbors = get_neighbors(x, y, matrix);

    if 2 <= num_neighbors && num_neighbors <= 3 {
        return true;
    } else {
        return false;
    }
}

fn check_dead(x: usize, y: usize, matrix: &Matrix<bool>) -> bool {
    let num_neighbors = get_neighbors(x, y, matrix);

    if num_neighbors == 3 {
        return true;
    } else {
        return false;
    }
}

fn get_neighbors(x: usize, y: usize, matrix: &Matrix<bool>) -> u8 {
    let mut total: u8 = 0;
    let diffs: [(i64, i64); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for (xd, yd) in diffs {
        let xn = x as i64 + xd;
        let yn = y as i64 + yd;

        if xn < 0 || yn < 0 {
            continue;
        }

        // xn and yn are derived from x and y which are usize so this is *probably* safe
        // also, if you're making a world that large, what's wrong with you?
        let Some(val) = matrix.get(xn as usize, yn as usize) else {
            continue;
        };

        total += *val as u8
    }

    total
}
