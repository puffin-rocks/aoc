use std::collections::HashMap;
use crate::utils::{assert_display, Label, Solve};

pub(crate) struct Advent {
    label: Label,
    messages: Vec<String>
}
impl Default for Advent {
    fn default() -> Self{
        Self{
            label: Label::new(6, 2016),
            messages: Vec::new()
        }
    }
}

impl Advent {
    fn solve(&self,
             most_common: bool,
             result_prd: &str,
             part: u8
    ) -> Result<String, String> {
        self.check_input(Some(part))?;
        let mut counter: HashMap<(char, usize), usize> = HashMap::new();
        for message in self.messages.iter(){
            for (i, ch) in message.chars().enumerate(){
                *counter.entry((ch, i)).or_insert(0)+=1;
            }
        }
        let mut message = String::new();
        for i in 0..self.messages[0].len(){
            let filter_map = counter.iter().filter_map(|((_ch, index), v)| if index==&i {Some(*v)} else{None});
            let freq = if most_common {
                filter_map.max().unwrap()
            }else{
                filter_map.min().unwrap()
            };
            let ch = counter.iter().filter_map(|((ch, index), v)| if index==&i && v == &freq {Some(*ch)} else{None}).take(1).collect::<Vec<_>>();
            message.push(ch[0]);
        }
        assert_display(message, None, String::from(result_prd), "Message", false)
    }
}
impl Solve for Advent {

    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn add_record_from_line(&mut self, line: String) -> Result<(), std::num::ParseIntError>{
        self.messages.push(line);
        Ok(())
    }

    fn info(&self) -> Result<(), String>{
        self.check_input(None)?;
        println!("Number of messages: {}", self.messages.len());
        Ok(())
    }
    fn compute_part1_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.solve(true, "afwlyyyq", 1)
    }
    fn compute_part2_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.solve(false, "bhkzekao", 2)
    }
}