use std::fs;

pub (crate) fn load_map(path: &str) -> Vec<Vec<char>> {
    let mut map: Vec<Vec<char>> = Vec::new();

    let map_file = fs::read_to_string(path).unwrap();
    for line in map_file.lines() {
        let tree_line: Vec<char> = line.chars().collect();
        map.push(tree_line);
    }

    return map;
}

pub (crate) fn print_map(map: Vec<Vec<char>>) -> () {
    for line in map {
        for character in line {
            print!("{}", character);
        }
        println!()
    }
}

pub (crate) fn count_trees(map: &Vec<Vec<char>>, x_step: usize, y_step: usize) -> u32 {
    assert!(map.len() > 0);

    let mut trees: u32 = 0;
    let mut x_pos: usize = 0;
    let mut y_pos: usize = 0;
    let height: usize = map.len();
    let width: usize = map[0].len();

    while y_pos < height {
        if map[y_pos][x_pos] == '#' {
            trees += 1;
        }

        x_pos = (x_pos + x_step) % width;
        y_pos += y_step;
    }

    return trees;
}

pub (crate) fn eval_slopes(map: &Vec<Vec<char>>, slopes: Vec<(usize, usize)>) -> u64 {
    let mut slope_trees: u64 = 1;

    for (x_step, y_step) in slopes {
        slope_trees *= count_trees(map, x_step, y_step) as u64;
    }

    return slope_trees
}