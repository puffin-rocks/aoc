use std::collections::{VecDeque};
use crate::utils::{assert_display, Label, Solve};


pub(crate) struct Advent {
    label: Label,
    input: String
}
impl Default for Advent {
    fn default() -> Self{
        Self{
            label: Label::new(9, 2017),
            input: String::new()
        }
    }
}

impl Solve for Advent {

    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn add_record_from_line(&mut self, line: String) -> Result<(), std::num::ParseIntError>{
        self.input = line;
        Ok(())
    }

    fn info(&self) -> Result<(), String>{
        self.check_input(None)?;
        println!("Input length: {}", self.input.len());
        Ok(())
    }
    fn compute_part1_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.check_input(Some(1))?;
        let mut cancel_mode = false;
        let mut garbage_mode = false;
        let mut current_level: usize = 0;
        let mut groups: VecDeque<usize> = VecDeque::new();
        let mut total_score = 0usize;
        for ch in self.input.chars(){
            if cancel_mode{
                cancel_mode = false;
                continue;
            }
            if garbage_mode{
                match ch{
                    '>' => garbage_mode = false,
                    '!' => cancel_mode = true,
                    _ => {}
                }
                continue;
            }
            match ch{
                '{' => {
                    current_level += 1;
                    groups.push_back(current_level);
                },
                '}' => {
                    current_level -= 1;
                    total_score += groups.pop_back().unwrap();
                },
                '<' => {
                    garbage_mode = true;
                },
                _ =>{}
            }
        }
        assert_display(total_score, None, 21037, "Total score", false)
    }
    fn compute_part2_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.check_input(Some(2))?;
        let mut cancel_mode = false;
        let mut garbage_mode = false;
        let mut total_garbage = 0usize;
        for ch in self.input.chars(){
            if cancel_mode{
                cancel_mode = false;
                continue;
            }
            if garbage_mode{
                match ch{
                    '>' => garbage_mode = false,
                    '!' => cancel_mode = true,
                    _ => {total_garbage+=1}
                }
                continue;
            }
            match ch{
                '<' => {
                    garbage_mode = true;
                },
                _ =>{}
            }
        }
        assert_display(total_garbage, None, 9495, "Total garbage", false)
    }
}