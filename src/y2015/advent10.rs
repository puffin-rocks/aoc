use crate::utils::{assert_display, Label, Solve};

pub(crate) struct Advent {
    label: Label,
    input: Vec<u8>
}
impl Default for Advent {
    fn default() -> Self{
        Self{
            label: Label::new(10, 2015),
            input: Vec::new()
        }
    }
}

impl Solve for Advent {

    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn add_record_from_line(&mut self, line: String) -> Result<(), std::num::ParseIntError>{
        self.input = line.chars().map(|x| x.to_digit(10).unwrap() as u8).collect();
        Ok(())
    }

    fn info(&self) -> Result<(), String>{
        self.check_input(None)?;
        println!("Input: {:?}", self.input);
        Ok(())
    }

    fn compute_part1_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.check_input(Some(1))?;
        let mut input = self.input.clone();
        let mut cnt = 0;
        while cnt<40{
            input = look_and_say(&input);
            cnt+=1;
        }
        assert_display(input.len(), None, 492982, "Sequence length", false)
    }

    fn compute_part2_answer(&self,  _: bool) -> Result<String, String>{
        self.check_input(Some(2))?;
        let mut input = self.input.clone();
        let mut cnt = 0;
        while cnt<50{
            input = look_and_say(&input);
            cnt+=1;
        }
        assert_display(input.len(), None, 6989950, "Sequence length", false)
    }
}

fn look_and_say(input: &Vec<u8>)->Vec<u8>{
    let mut result:String = String::new();
    let mut current:u8 = 0;
    let mut cnt: usize = 0;
    for n in input.iter(){
        if current!=*n{
            if cnt>0{
                result.push_str(format!("{}{}",cnt, current).as_str());
            }
            current = *n;
            cnt=1;
        }else{
            cnt+=1;
        }
    }
    result.push_str(format!("{}{}",cnt, current).as_str());
    result.chars().map(|x| x.to_digit(10).unwrap() as u8).collect()
}