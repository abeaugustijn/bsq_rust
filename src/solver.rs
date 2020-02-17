use super::SolveRes;

/// A simple function to determine the smallest of 3 integers
///
/// @param {u16} x
/// @param {u16} y
/// @param {u16} z
///
/// @return {u16}

fn min(x: u16, y: u16, z: u16) -> u16 {
    if x <= y && x <= z {
        return x;
    }
    if y <= x && y <= z {
        return y;
    }
    if z <= x && z <= y {
        return z;
    }
    return x;
}

/// This solves the map
///
/// @param {&mut Vec<Vec<u16>>} map - a mutable reference to the map
///
/// @return {SolveRes} - a struct containing the result of the solve

pub fn solver(map: &mut Vec<Vec<u16>>) -> SolveRes {
    let mut res = SolveRes {
        x: 0,
        y: 0,
        size: 0,
    };

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if i == 0 || j == 0 || map[i][j] == 0 {
                continue;
            }
            map[i][j] = map[i][j] + min(map[i - 1][j], map[i - 1][j - 1], map[i][j - 1]);
            if map[i][j] > res.size {
                res.x = j as u16;
                res.y = i as u16;
                res.size = map[i][j];
            }
        }
    }
    return res;
}
