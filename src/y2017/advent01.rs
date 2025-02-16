use crate::utils::{assert_display, Label, Solve};

pub(crate) struct Advent {
    label: Label,
    line: String
}
impl Default for Advent {
    fn default() -> Self{
        Self{
            label: Label::new(1, 2017),
            line: String::new()
        }
    }
}

impl Solve for Advent {

    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn add_record_from_line(&mut self, line: String) -> Result<(), std::num::ParseIntError>{
        self.line = line;
        Ok(())
    }

    fn info(&self) -> Result<(), String>{
        self.check_input(None)?;
        println!("Input length: {}", self.line.len());
        Ok(())
    }
    fn compute_part1_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.check_input(Some(1))?;
        let mut sum = self.line.chars().collect::<Vec<_>>().windows(2).filter_map(|ch|
            if ch[0] == ch[1]{
                ch[0].to_digit(10)
            }else{
                None
            }
        ).sum::<u32>();
        if self.line.chars().nth(self.line.len()-1) == self.line.chars().nth(0){
            sum+=self.line.chars().nth(self.line.len()-1).unwrap().to_digit(10).unwrap();
        }
        assert_display(sum, None, 1031, "Sum", false)
    }
    fn compute_part2_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.check_input(Some(2))?;
        let n = self.line.len();
        let shift = n/2;
        let sum = self.line.chars().enumerate().filter_map(|(i, ch)|
            if ch == self.line.chars().nth((i+shift)%n).unwrap(){
                ch.to_digit(10)
            }else{
                None
            }
        ).sum::<u32>();
        assert_display(sum, None, 1080, "Sum", false)
        //Err(String::from("Not implemented"))
    }
}