use crate::utils::{assert_display, Label, Solve};
use regex::Regex;

pub(crate) struct Advent {
    label: Label,
    row: usize,
    col: usize
}
impl Default for Advent {
    fn default() -> Self{
        Self{
            label: Label::new(25, 2015),
            row: 0,
            col: 0
        }
    }
}

impl Solve for Advent {

    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn add_record_from_line(&mut self, line: String) -> Result<(), std::num::ParseIntError>{
        let re = Regex::new(r"To continue, please consult the code grid in the manual.  Enter the code at row (\d+), column (\d+).").unwrap();
        if let Some(captures) = re.captures(line.as_str()) {
            self.row = captures.get(1).unwrap().as_str().parse::<usize>()?;
            self.col = captures.get(2).unwrap().as_str().parse::<usize>()?;
        }
        Ok(())
    }

    fn info(&self) -> Result<(), String>{
        self.check_input(None)?;
        println!("Coordinates {:?}", (self.row, self.col));
        Ok(())
    }
    fn compute_part1_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.check_input(Some(1))?;
        let mut value: usize = 20151125;
        let (mut row, mut col) = (0, 0);
        loop{
            (row, col) = next_coordinate(row, col);
            value = next_code(value);
            if row ==self.row-1 && col == self.col-1{
                break;
            }
        }
        assert_display(value, None, 2650453, "Code", false)
    }
    fn compute_part2_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.check_input(Some(2))?;
        Ok(String::from("No computation required"))
    }
}

fn next_code(value: usize)->usize{
    (value*252533)%33554393
}
fn next_coordinate(row:usize, col:usize) -> (usize, usize){
    if row ==0{
        (col+1, 0)
    }else{
        (row-1, col+1)
    }
}