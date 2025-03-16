use std::collections::{HashMap, VecDeque};
use crate::utils::{assert_display, Label, Solve};

pub(crate) struct Advent {
    label: Label,
    instructions: Vec<Vec<String>>
}
impl Default for Advent {
    fn default() -> Self{
        Self{
            label: Label::new(18, 2017),
            instructions: Vec::new()
        }
    }
}

impl Solve for Advent {

    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn add_record_from_line(&mut self, line: String) -> Result<(), std::num::ParseIntError>{
        self.instructions.push(line.split_whitespace().map(|x| String::from(x)).collect::<Vec<_>>());
        Ok(())
    }

    fn info(&self) -> Result<(), String>{
        self.check_input(None)?;
        println!("Number of instructions: {}", self.instructions.len());
        Ok(())
    }
    fn compute_part1_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.check_input(Some(1))?;
        let n = self.instructions.len() as isize;
        let mut registers: HashMap<char, isize> = HashMap::new();
        let mut pos = 0isize;
        let mut last_freq: Option<isize> = None;
        let mut rec_freq: Option<isize> = None;
        while pos>-1 && pos< n && rec_freq.is_none() {
            pos = process_instruction(pos, &self.instructions[pos as usize], &mut registers, &mut last_freq, &mut rec_freq, &mut None, &mut None, &mut None);
        }
        match rec_freq{
            Some(result) =>assert_display(result, None, 7071, "Recovered frequency", false),
            None => Err(String::from("Not implemented"))
        }
    }
    fn compute_part2_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.check_input(Some(2))?;
        let n = self.instructions.len() as isize;
        let mut registers0: HashMap<char, isize> = HashMap::new();
        let mut registers1: HashMap<char, isize> = HashMap::new();
        registers1.insert('p',1);
        let mut pos0 = 0isize;
        let mut pos1 = 0isize;
        let mut last_freq: Option<isize> = None;
        let mut rec_freq: Option<isize> = None;
        let mut pipe0: Option<VecDeque<isize>> = Some(VecDeque::new());
        let mut pipe1: Option<VecDeque<isize>> = Some(VecDeque::new());
        let mut snd_cnt: Option<usize> = Some(0);
        while (pos0>-1 && pos0<n) || (pos1>-1 && pos1<n) {
            if pos0>-1 && pos0<n {
                pos0 = process_instruction(pos0, &self.instructions[pos0 as usize], &mut registers0, &mut last_freq, &mut rec_freq, &mut pipe1, &mut pipe0, &mut None);
            }
            if pos1>-1 && pos1<n {
                pos1 = process_instruction(pos1, &self.instructions[pos1 as usize], &mut registers1, &mut last_freq, &mut rec_freq, &mut pipe0, &mut pipe1, &mut snd_cnt);
            }
            if (pos0>-1 && pos0<n) && (pos1>-1 && pos1<n) {
                if &self.instructions[pos0 as usize].as_slice()[0] == "rcv" &&
                    &self.instructions[pos1 as usize].as_slice()[0] == "rcv" &&
                    pipe0.clone().unwrap().is_empty() &&
                    pipe1.clone().unwrap().is_empty()
                {
                    break;
                }
            }
        }
        match snd_cnt{
            Some(result) =>assert_display(result, None, 8001, "Number of program 1's sending", false),
            None => Err(String::from("Not implemented"))
        }
    }
}

fn get_value(arg: &String, registers: &HashMap<char,isize>) -> isize{
    match arg.parse::<isize>() {
        Ok(value) => value,
        Err(_) => {
            let reg_key = arg.chars().nth(0).unwrap();
            let value = registers.get(&reg_key).unwrap_or(&0);
            *value
        }
    }
}

fn process_instruction(pos: isize,
                       instruction: &Vec<String>,
                       registers: &mut HashMap<char, isize>,
                       last_freq: &mut Option<isize>,
                       rec_freq: &mut Option<isize>,
                       pipe_snd: &mut Option<VecDeque<isize>>,
                       pipe_rcv: &mut Option<VecDeque<isize>>,
                       snd_cnt: &mut Option<usize>
) ->isize{
    match instruction.as_slice() {
        [op, arg0, arg1] => {
            match op.as_str(){
                "set" | "mod" => {
                    match arg0.parse::<isize>() {
                        Ok(_) => unreachable!(),
                        Err(_) => {
                            let key = arg0.chars().nth(0).unwrap();
                            let value = if op == &"set"{
                                get_value(arg1, &registers)
                            }else{
                                get_value(arg0, &registers)%get_value(arg1, &registers)
                            };
                            registers.insert(key, value);
                            pos+1
                        }
                    }
                },
                "mul"| "add" => {
                    match arg0.parse::<isize>() {
                        Ok(_) => unreachable!(),
                        Err(_) => {
                            let key = arg0.chars().nth(0).unwrap();
                            if op ==&"mul" {
                                *registers.entry(key).or_insert(0) *= get_value(arg1, &registers);
                            }else{
                                *registers.entry(key).or_insert(0) += get_value(arg1, &registers);
                            }
                            pos+1
                        }
                    }
                }
                "jgz" => {
                    if get_value(arg0, &registers)>0{
                        pos+get_value(arg1, &registers)
                    }else{
                        pos+1
                    }
                }
                _ => {
                    println!("{:?}", (op, arg0, arg1));
                    todo!()
                }
            }
        },
        [op, arg] => {
            match op.as_str(){
                "snd" =>{
                    match pipe_snd{
                        Some(v) => {
                            v.push_back(get_value(arg, &registers));
                            if let Some(v) = snd_cnt{
                                *v+=1;
                            }
                        },
                        None => {
                            *last_freq = Some(get_value(arg, &registers));
                        }
                    }
                    pos + 1
                }
                "rcv" =>{
                    match pipe_rcv{
                        Some(v) => {
                            match arg.parse::<isize>() {
                                Ok(_) => unreachable!(),
                                Err(_) => {
                                    if let Some(value) = v.pop_front(){
                                        let key = arg.chars().nth(0).unwrap();
                                        registers.insert(key, value);
                                        pos+1
                                    }else{
                                        pos
                                    }
                                }
                            }
                        },
                        None => {
                            if get_value(arg, &registers) != 0 {
                                *rec_freq = *last_freq;
                            }
                            pos + 1
                        }
                    }
                }
                _ => {
                    println!("{:?}", (op, arg));
                    todo!()
                }
            }
        },
        _ => unreachable!()
    }
}