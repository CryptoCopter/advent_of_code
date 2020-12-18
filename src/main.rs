mod toboggan;

fn main() {
    let toboggan_map = toboggan::load_map("input/trees.txt");
    let slopes: Vec<(usize, usize)> = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    println!("Number of trees encountered: {}", toboggan::count_trees(&toboggan_map, 3, 1));
    println!("Cumulative trees for all slopes: {}", toboggan::eval_slopes(&toboggan_map, slopes));
}
