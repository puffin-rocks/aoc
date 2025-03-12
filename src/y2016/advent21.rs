use regex::Regex;
use crate::utils::{assert_display, swap_vec_elements, Label, Solve};

#[derive(Debug)]
enum Operation{
    SwapPosition(usize,usize),
    SwapValue(char,char),
    RotateRight(bool,usize),
    RotateValue(char),
    Reverse(usize,usize),
    Move(usize,usize)
}

impl Operation {
    fn apply(&self, password: &mut Vec<char>, revert: bool){
        let n = password.len();
        match &self{
            Operation::SwapPosition(p1, p2) =>{
                if p2>p1 {
                    let (left, right) = password.split_at_mut(*p2);
                    std::mem::swap(&mut left[*p1], &mut right[0]);
                }else{
                    let (left, right) = password.split_at_mut(*p1);
                    std::mem::swap(&mut left[*p2], &mut right[0]);
                }
            }
            Operation::SwapValue(ch1, ch2) =>{
                if let (Some(p1), Some(p2)) = (password.iter().position(|x| x == ch1), password.iter().position(|x| x == ch2)) {
                    swap_vec_elements::<char>(password, p1, p2);
                }
            }
            Operation::RotateRight(is_right, n_steps) => {
                if *is_right ^ revert {
                    password.rotate_right(*n_steps%n)
                }else{
                    password.rotate_left(*n_steps%n);
                }
            }
            Operation::RotateValue(ch) => {
                if let Some(pos) = password.iter().position(|x| x == ch) {
                    if revert{
                        let n_steps = match pos {
                            0 => 9,
                            1 => 1,
                            2 => 6,
                            3 => 2,
                            4 => 7,
                            5 => 3,
                            6 => 8,
                            7 => 4,
                            _=> unreachable!()
                        };
                        password.rotate_left(n_steps%n)
                    }else {
                        let n_steps = if pos > 3 {
                            pos + 2
                        } else {
                            pos + 1
                        };
                        password.rotate_right(n_steps%n)
                    }
                }else{
                    unreachable!()
                }
            }
            Operation::Reverse(s, e) => {
                password[*s..=*e].reverse();
            }
            Operation::Move(from, to) => {
                let (mut from, mut to) = (*from, *to);
                if revert{
                    std::mem::swap(&mut from, &mut to);
                }
                let ch = password.remove(from);
                password.insert(to, ch);
            }
        }
    }
}

pub(crate) struct Advent {
    label: Label,
    operations: Vec<Operation>
}
impl Default for Advent {
    fn default() -> Self{
        Self{
            label: Label::new(21, 2016),
            operations: Vec::new()
        }
    }
}

impl Solve for Advent {

    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn add_record_from_line(&mut self, line: String) -> Result<(), std::num::ParseIntError>{
        let re = Regex::new(r"rotate (\w+) (\d+) steps?").unwrap();
        if let Some(captures) = re.captures(line.as_str()) {
            let is_right = match captures.get(1).unwrap().as_str() {
                "right" => true,
                "left" => false,
                _ => unreachable!()
            };
            let n_steps = captures.get(2).unwrap().as_str().parse::<usize>()?;
            self.operations.push(Operation::RotateRight(is_right, n_steps));
            return Ok(())
        }
        let re = Regex::new(r"swap position (\d+) with position (\d+)").unwrap();
        if let Some(captures) = re.captures(line.as_str()) {
            let p1 = captures.get(1).unwrap().as_str().parse::<usize>()?;
            let p2 = captures.get(2).unwrap().as_str().parse::<usize>()?;
            self.operations.push(Operation::SwapPosition(p1,p2));
            return Ok(())
        }
        let re = Regex::new(r"rotate based on position of letter (\w+)").unwrap();
        if let Some(captures) = re.captures(line.as_str()) {
            let letter = captures.get(1).unwrap().as_str().chars().nth(0).unwrap();
            self.operations.push(Operation::RotateValue(letter));
            return Ok(())
        }
        let re = Regex::new(r"swap letter (\w+) with letter (\w+)").unwrap();
        if let Some(captures) = re.captures(line.as_str()) {
            let v1 = captures.get(1).unwrap().as_str().chars().nth(0).unwrap();
            let v2 = captures.get(2).unwrap().as_str().chars().nth(0).unwrap();
            self.operations.push(Operation::SwapValue(v1,v2));
            return Ok(())
        }
        let re = Regex::new(r"reverse positions (\d+) through (\d+)").unwrap();
        if let Some(captures) = re.captures(line.as_str()) {
            let p1 = captures.get(1).unwrap().as_str().parse::<usize>()?;
            let p2 = captures.get(2).unwrap().as_str().parse::<usize>()?;
            self.operations.push(Operation::Reverse(p1,p2));
            return Ok(())
        }
        let re = Regex::new(r"move position (\d+) to position (\d+)").unwrap();
        if let Some(captures) = re.captures(line.as_str()) {
            let p1 = captures.get(1).unwrap().as_str().parse::<usize>()?;
            let p2 = captures.get(2).unwrap().as_str().parse::<usize>()?;
            self.operations.push(Operation::Move(p1,p2));
            return Ok(())
        }
        Ok(())
    }

    fn info(&self) -> Result<(), String>{
        self.check_input(None)?;
        println!("Number of operations: {}", self.operations.len());
        Ok(())
    }
    fn compute_part1_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.check_input(Some(1))?;
        let mut password: Vec<char> = "abcdefgh".chars().collect::<Vec<_>>();
        for op in self.operations.iter(){
            op.apply(&mut password, false);
        }
        let result =  password.iter().collect::<String>();
        assert_display(result, None, String::from("baecdfgh"), "Scrambled passport", false)
    }
    fn compute_part2_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.check_input(Some(2))?;
        let mut password: Vec<char> = "fbgdceah".chars().collect::<Vec<_>>();
        for op in self.operations.iter().rev(){
            op.apply(&mut password,true);
        }
        let result =  password.iter().collect::<String>();
        assert_display(result, None, String::from("cegdahbf"), "Unscrambled passport", false)
    }
}