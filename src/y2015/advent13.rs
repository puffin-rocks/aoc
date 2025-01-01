use crate::utils::{assert_display, Label, Solve};

pub(crate) struct Advent {
    label: Label
}
impl Default for Advent {
    fn default() -> Self{
        Self{
            label: Label::new(0, 2015)
        }
    }
}

impl Solve for Advent {

    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn add_record_from_line(&mut self, _line: String) -> Result<(), std::num::ParseIntError>{
        "invalid".parse::<i32>()?;
        Ok(())
    }

    fn info(&self) -> Result<(), String>{
        self.check_input(None)?;
        Ok(())
    }

    // fn compute_part1_answer(&self, _test_mode: bool) -> Result<String, String>{
    //     self.check_input(Some(1))?;
    //     Err(String::from("Part 1 not implemented yet"))
    // }
    //
    // fn compute_part2_answer(&self,  _: bool) -> Result<String, String>{
    //     self.check_input(Some(2))?;
    //     Err(String::from("Part 2 not implemented yet"))
    // }
}