use solver::Solver;

mod solver;

fn main() {
    let puzzle_string = "1.5..2.84..63.12.7.2..5.....9..1....8.2.3674.3.7.2..9.47...8..1..16....926914.37.";
    let puzzle_string2 = "..............3.85..1.2.......5.7.....4...1...9.......5......73..2.1........4...9";
    let mut puzzle = Solver::new(puzzle_string);
    puzzle.solve();
    println!("{}", puzzle.current_string() )
}
