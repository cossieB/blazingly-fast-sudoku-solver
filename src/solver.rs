use regex::Regex;
#[derive(Debug)]
pub struct Cell {
    value: char,
    row: u8,
    column: u8,
    region: u8,
    frozen: bool,
    cell_number: u8,
}
pub enum ValidationRes {
    Good,
    Bad(String)
}
pub struct Solver {
    puzzle_string: String,
    pub array: Vec<Cell>,
}

impl Solver {
    pub fn validate(puzzle_string: &str) -> ValidationRes {
        if puzzle_string.len() != 81 {
            return ValidationRes::Bad("Expected string to be 81 characters".to_string());
        }
        let re = Regex::new(r"[^\d\.]").unwrap();
        if re.is_match(puzzle_string) {
            return ValidationRes::Bad("Invalid characters".to_string())
        }
        return ValidationRes::Good
    }

    pub fn new(puzzle_string: &str) -> Solver {
        let result = Solver::validate(puzzle_string);
        match result {
            ValidationRes::Bad(err) => panic!("{err}"),
            ValidationRes::Good => {
                Solver {
                    puzzle_string: puzzle_string.to_string(),
                    array: Solver::get_array(puzzle_string)
                }
            }
        }
    }
    fn get_array(puzzle_string: &str) -> Vec<Cell> {
        let mut arr: Vec<Cell> = vec![];
        for (i, value) in puzzle_string.char_indices() {
            let row = (i as f64 / 9.0) as u8;
            let column = (i as f64 % 9.0) as u8;

            let region: u8;

            if row < 3 && column < 3 { region = 0 }
            else if row < 3 && column < 6 {region = 1}
            else if row < 3 && column < 9 {region = 2}
            else if row < 6 && column < 3 {region = 3}
            else if row < 6 && column < 6 {region = 4}
            else if row < 6 && column < 9 {region = 5}
            else if row < 9 && column < 3 {region = 6}
            else if row < 9 && column < 6 {region = 7}
            else {region = 8}

            arr.push(Cell {
                value,
                row,
                column,
                region,
                frozen: value != '.',
                cell_number: i as u8
            }) 
        }
        arr
    }
    pub fn solve(&mut self) {
        let mut blanks = vec![];
        for cell in &self.array {
            if !cell.frozen {
                cell.value = '.';
                blanks.push(cell)
            }
        }
        let position = 0;
        let mut direction = 0i8;

        loop {
            let cell = blanks[position];
            let result = self.place_number(cell.cell_number, direction);
            direction = if result == '.' {-1} else {1};
            cell.value = result;
        }
    }
    fn place_number(&self, cell_number: u8, direction: i8) -> char {
        let cell = &self.array[cell_number as usize];
        let mut num = if direction == 1 {1} else {cell.value.to_digit(10).unwrap()};

        while num <=9 {
            let mut check_column = true;
            let mut check_row = true;
            let mut check_region = true;
            
            for item in &self.array {
                if item.column == cell.column && item.value.to_string() == num.to_string() {
                    check_column = false;
                }
                if item.row == cell.row && item.value.to_string() == num.to_string() {
                    check_row = false;
                }
                if item.region == cell.region && item.value.to_string() == num.to_string() {
                    check_region = false;
                }
            }
            if check_region && check_column && check_row {
                return num.to_string().chars().next().unwrap()
            }
            num += 1;
        }
        return '.'
    }
}
