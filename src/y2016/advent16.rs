use std::collections::{VecDeque};
use crate::utils::{assert_display, Label, Solve};

pub(crate) struct Advent {
    label: Label,
    start: String,
}
impl Default for Advent {
    fn default() -> Self{
        Self{
            label: Label::new(16, 2016),
            start: String::new()
        }
    }
}

impl Advent {
    fn solve(&self,
             disc_size: usize,
             result_prd: String,
             part: u8
    ) -> Result<String, String> {
        self.check_input(Some(part))?;

        let mut a = vec!['a'];
        while length(&a, self.start.len()) < disc_size {
            a = stitch(&a, &mirror(&a));
        }

        let mut stack: VecDeque<char> = VecDeque::new();
        let mut a_iter = a.iter();

        let a_full = self.start.clone();
        let b_full = mirror_string(&self.start);
        let mut checksum: Vec<char> = Vec::new();
        while checksum.len()<disc_size{
            if stack.len()<2 {
                let ch = a_iter.next().unwrap();
                match ch {
                    &'1' |  &'0' => stack.push_back(*ch),
                    &'a' => a_full.chars().for_each(|x| stack.push_back(x)),
                    &'b' => b_full.chars().for_each(|x| stack.push_back(x)),
                    _ => unreachable!()
                };
            }
            checksum.push(stack.pop_front().unwrap());
        }

        while checksum.len()%2 ==0 {
            checksum = checksum
                .chunks_exact(2)
                .map(|w| if w[0] == w[1] { '1' } else { '0' })
                .collect();
        }
        assert_display(checksum.iter().collect::<String>(), None, result_prd, "Checksum", false)

    }
}

impl Solve for Advent {

    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn add_record_from_line(&mut self, line: String) -> Result<(), std::num::ParseIntError>{
        self.start = line;
        Ok(())
    }

    fn info(&self) -> Result<(), String>{
        self.check_input(None)?;
        println!("Start: {}", self.start);
        Ok(())
    }
    fn compute_part1_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.solve(272, String::from("10010101010011101"), 1)
    }
    fn compute_part2_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.solve(35651584, String::from("01100111101101111"), 2)
    }
}

fn length(a: &Vec<char>, a_size: usize) -> usize{
    let n_a = (a.len()+1)/2;
    n_a*a_size + n_a-1
}

fn mirror(a: &Vec<char>) ->Vec<char>{
    a.iter().rev()
        .map(|x| match x{
            '1' => '0',
            '0' => '1',
            'a' => 'b',
            'b' => 'a',
            _ => unreachable!()
        }).collect::<Vec<_>>()
}

fn mirror_string(a: &String) ->String{
    a.chars().rev()
        .map(|x| match x{
            '1' => '0',
            '0' => '1',
            _ => unreachable!()
        }).collect::<String>()
}

fn stitch(a:&Vec<char>, b:&Vec<char>) ->Vec<char>{
    a.iter().chain(['0'].iter()).chain(b.iter()).cloned().collect::<Vec<char>>()
}