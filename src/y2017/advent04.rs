use std::collections::HashSet;
use crate::utils::{assert_display, Label, Solve};

pub(crate) struct Advent {
    label: Label,
    passphrases: Vec<Vec<String>>
}
impl Default for Advent {
    fn default() -> Self{
        Self{
            label: Label::new(4, 2017),
            passphrases: Vec::new()
        }
    }
}

impl Solve for Advent {

    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn add_record_from_line(&mut self, line: String) -> Result<(), std::num::ParseIntError>{
        self.passphrases.push(line.split_whitespace().map(|x| String::from(x)).collect::<Vec<String>>());
        Ok(())
    }

    fn info(&self) -> Result<(), String>{
        self.check_input(None)?;
        println!("Number of passphrases: {}", self.passphrases.len());
        Ok(())
    }
    fn compute_part1_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.check_input(Some(1))?;
        let n = self.passphrases.iter()
            .filter(|&x| x.iter().collect::<HashSet<_>>().len() == x.len())
            .count();
        assert_display(n, None, 466, "Number of valid passphrases", false)
    }
    fn compute_part2_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.check_input(Some(2))?;
        let n = self.passphrases.iter()
            .map(|x| x.iter().map(|y| string2array(y)).collect::<Vec<_>>())
            .filter(|x| x.iter().collect::<HashSet<_>>().len() == x.len())
            .count();
        assert_display(n, None, 251, "Number of valid passphrases", false)
    }
}

fn string2array(input: &String)->[usize; 26]{
    let mut result: [usize; 26] = [0;26];
    for ch in input.chars(){
        let idx = ch as usize - 'a' as usize;
        result[idx]+=1;
    }
    result
}