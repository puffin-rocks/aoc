use std::collections::HashSet;
use itertools::Itertools;
use crate::utils::{assert_display, Label, Solve};

pub(crate) struct Advent {
    label: Label,
    input: Vec<char>
}
impl Default for Advent {
    fn default() -> Self{
        Self{
            label: Label::new(11, 2015),
            input: Vec::new()
        }
    }
}

impl Solve for Advent {

    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn add_record_from_line(&mut self, line: String) -> Result<(), std::num::ParseIntError>{
        self.input = line.chars().collect();
        Ok(())
    }

    fn info(&self) -> Result<(), String>{
        self.check_input(None)?;
        println!("{:?}", self.input);
        Ok(())
    }

    fn compute_part1_answer(&self, _: bool) -> Result<String, String>{
        self.check_input(Some(1))?;
        match gen_next_correct_password(self.input.clone()){
            Some(input)=> assert_display(input.iter().join(""),
                                         None,
                                         String::from("hepxxyzz"),
                                         "Password", false),
            None => Err(String::from("No solution found"))
        }
    }

    fn compute_part2_answer(&self,  _: bool) -> Result<String, String>{
        self.check_input(Some(2))?;
        match gen_next_correct_password(self.input.clone()){
            Some(mut input)=> {
                gen_next_password(&mut input);
                match gen_next_correct_password(input){
                    Some(input)=> assert_display(input.iter().join(""),
                                                 None,
                                                 String::from("heqaabcc"),
                                                 "Password", false),
                    None => Err(String::from("No solution found"))
                }
            },
            None => Err(String::from("No solution found"))
        }
    }
}

fn gen_next_correct_password(mut input: Vec<char>)->Option<Vec<char>>{
    let forbidden = vec!['i','l','o'];
    let alphabet: Vec<char> = ('a'..='z').collect();
    let password_len = input.len();
    loop {
        if input==vec!['z';password_len]{
            return None
        }
        // Check for forbidden characters
        if input.iter().any(|&ch| forbidden.contains(&ch)) {
            gen_next_password(&mut input);
            continue;
        }

        // Check for a straight of three consecutive letters
        if !alphabet.windows(3).any(|seq| input.windows(3).any(|win| win == seq)) {
            gen_next_password(&mut input);
            continue;
        }

        // Check for two different, non-overlapping pairs
        let pairs: HashSet<_> = input
            .windows(2)
            .filter(|&pair| pair[0] == pair[1])
            .map(|pair| pair[0])
            .collect();
        if pairs.len() < 2 {
            gen_next_password(&mut input);
            continue;
        }

        return Some(input);
    }
}

fn gen_next_password(input: &mut Vec<char>) {
    for ch in input.iter_mut().rev() {
        if *ch == 'z' {
            *ch = 'a';
        } else {
            *ch = ((*ch as u8) + 1) as char;
            break;
        }
    }
}