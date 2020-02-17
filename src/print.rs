use super::SolveRes;

/// Print out the solved map
///
/// @param {Vec<Vec<u16>>} map
/// @param {SolveRes} solve_res

pub fn print_map(map: Vec<Vec<u16>>, solve_res: SolveRes) {
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if j as u16 > solve_res.x - solve_res.size
                && i as u16 > solve_res.y - solve_res.size
                && (j as u16) < solve_res.x
                && (i as u16) < solve_res.y
            {
                print!("X");
            } else {
                print!(
                    "{}",
                    match map[i][j] {
                        0 => 'o',
                        _ => '.',
                    }
                );
            }
        }
        print!("\n");
    }
}
