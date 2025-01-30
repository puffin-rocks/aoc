extern crate crypto;
use std::collections::VecDeque;
use crypto::md5::Md5;
use crypto::digest::Digest;
use crate::utils::{assert_display, Label, Solve};

pub(crate) struct Advent {
    label: Label,
    salt: String,
}
impl Default for Advent {
    fn default() -> Self{
        Self{
            label: Label::new(14, 2016),
            salt: String::new()
        }
    }
}

impl Advent {
    fn solve(&self,
             n_hash: usize,
             result_prd: usize,
             part: u8
    ) -> Result<String, String> {
        self.check_input(Some(part))?;
        let mut hasher = Md5::new();
        let mut n_revealed = 0;
        let mut queue: VecDeque<String> = VecDeque::new();
        for i in 0..=1000{
            let key = compute_n_hash(&mut hasher, format!("{}{}", self.salt, i), n_hash);
            queue.push_back(key);
        }
        let mut result: Option<usize> = None;
        for i in 1001..std::u64::MAX {
            let curr_key = queue.pop_front().unwrap();
            if let Some(ch ) = first_repeated_char(&*curr_key, 3){
                let pattern = ch.to_string().repeat(5);
                for s in queue.iter() {
                    if s.contains(&pattern){
                        n_revealed+=1;
                        break;
                    }
                }
            }
            if n_revealed==64{
                result = Some(i as usize-1001);
                break;
            }
            let key = compute_n_hash(&mut hasher, format!("{}{}", self.salt, i), n_hash);
            queue.push_back(key);
        }
        match result{
            Some(value) => assert_display(value, None,result_prd, "Index", false),
            None=>Err(String::from("No solution found"))
        }
    }
}

impl Solve for Advent {

    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn add_record_from_line(&mut self, line: String) -> Result<(), std::num::ParseIntError>{
        self.salt = line;
        Ok(())
    }

    fn info(&self) -> Result<(), String>{
        self.check_input(None)?;
        println!("Salt {}", self.salt);
        Ok(())
    }
    fn compute_part1_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.solve(1, 25427, 1)
    }
    fn compute_part2_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.solve(2017, 22045, 2)
    }
}

fn first_repeated_char(s: &str, n: usize) -> Option<char> {
    s.chars()
        .collect::<Vec<_>>()
        .windows(n)
        .find(|w| w.iter().all(|&c| c == w[0]))
        .map(|w| w[0])
}

fn compute_n_hash(hasher: &mut Md5, input: String, n: usize) -> String{
    let mut input = input;
    let mut i = 0;
    loop {
        if i==n{
            break;
        }
        hasher.input(input.as_bytes());
        let mut output = [0; 16]; // An MD5 is 16 bytes
        hasher.result(&mut output);
        input = hex::encode(output);
        hasher.reset();
        i+=1;
    }
    input
}