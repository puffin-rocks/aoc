use crate::utils::{assert_display, Label, Solve};

pub(crate) struct Advent {
    label: Label,
    starting_row: String
}
impl Default for Advent {
    fn default() -> Self{
        Self{
            label: Label::new(18, 2016),
            starting_row: String::new()
        }
    }
}

impl Advent{
fn solve(&self,
         row_limit: usize,
         result_prd: usize,
         part: u8
) -> Result<String, String> {
    self.check_input(Some(part))?;
    let n = self.starting_row.len();
    let mut current_row: Vec<char> = self.starting_row.chars().collect();
    let mut next_row = vec!['.'; n];
    let mut n_safe = current_row.iter().filter(|&&x| x == '.').count();
    let mut n_rows = 1;

    while n_rows < row_limit {
        let mut n_safe_row = 0;
        for i in 0..n {
            let (l, c, r) = (
                if i > 0 { current_row[i - 1] } else { '.' },
                current_row[i],
                if i < n - 1 { current_row[i + 1] } else { '.' },
            );
            next_row[i] = if matches!((l, c, r), ('^', '^', '.') | ('.', '^', '^') | ('.', '.', '^') | ('^', '.', '.')) {
                '^'
            } else {
                n_safe_row += 1;
                '.'
            };
        }
        n_safe += n_safe_row;
        std::mem::swap(&mut current_row, &mut next_row);
        n_rows += 1;
    }
    assert_display(n_safe, None, result_prd, "Number of safe tiles", false)
}
}

impl Solve for Advent {

    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn add_record_from_line(&mut self, line: String) -> Result<(), std::num::ParseIntError>{
        self.starting_row = line;
        Ok(())
    }

    fn info(&self) -> Result<(), String>{
        self.check_input(None)?;
        println!("Row length {}", self.starting_row.len());
        Ok(())
    }
    fn compute_part1_answer(&self, _test_mode: bool) -> Result<String, String> {
        self.solve(40, 1956,1)
    }
    fn compute_part2_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.solve(400_000, 19995121,2)
    }
}