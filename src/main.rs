use solver::Solver;

mod solver;

fn main() {
    let puzzle_string = "1.5..2.84..63.12.7.2..5.....9..1....8.2.3674.3.7.2..9.47...8..1..16....926914.37.";
    let puzzle = Solver::new(puzzle_string);

    println!("{:#?}", puzzle.array)
}
