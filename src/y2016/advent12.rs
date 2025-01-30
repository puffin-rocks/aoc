use std::collections::{HashMap, HashSet};
use crate::hashset;
use crate::utils::{assert_display, Label, Solve};

#[derive(Debug, Clone)]
enum Instruction {
    CPY,
    INC,
    DEC,
    JNZ,
}

#[derive(Debug, Clone)]
struct Command {
    instruction: Instruction,
    register_source: Option<char>,
    register_target: Option<char>,
    value: Option<isize>,
    offset: Option<isize>,
}

impl Command {
    fn new(instruction: Instruction,
           register_source: Option<char>,
           register_target: Option<char>,
           value: Option<isize>,
           offset: Option<isize>) -> Self {
        Self {
            instruction,
            register_source,
            register_target,
            value,
            offset,
        }
    }

    fn run(&self, index: usize, registers: &mut HashMap<char, isize>) -> usize {
        match self.instruction {
            Instruction::CPY | Instruction::JNZ  => {
                let value = if self.register_source.is_none() {
                    self.value.unwrap()
                } else {
                    let ch = self.register_source.unwrap();
                    *registers.get(&ch).unwrap_or(&0)
                };
                match self.instruction{
                    Instruction::CPY=>{
                        //println!("{:?}", &self);
                        registers.insert(self.register_target.unwrap(), value);
                        index + 1
                    },
                    Instruction::JNZ => {
                        if value != 0 {
                            (index as isize + self.offset.unwrap()) as usize
                        } else {
                            index + 1
                        }
                    },
                    _ => index + 1
                }

            }
            Instruction::DEC => {
                *registers.entry(self.register_target.unwrap()).or_insert(0) -= 1;
                index + 1
            }
            Instruction::INC => {
                *registers.entry(self.register_target.unwrap()).or_insert(0) += 1;
                index + 1
            }
        }
    }
}

pub(crate) struct Advent {
    label: Label,
    program: Vec<Command>,
    register_keys: HashSet<char>,
}
impl Default for Advent {
    fn default() -> Self {
        Self {
            label: Label::new(12, 2016),
            program: Vec::new(),
            register_keys: hashset!['a', 'b', 'c', 'd'],
        }
    }
}

impl Advent {
    fn solve(&self,
             registers: &mut HashMap<char, isize>,
             result_prd: isize,
             part: u8,
    ) -> Result<String, String> {
        self.check_input(Some(part))?;
        let mut index = 0;
        while index < self.program.len() {
            index = self.program.get(index).unwrap().run(index, registers);
        }
        assert_display(*registers.get(&'a').unwrap(), None, result_prd, "Value in 'a'", false)
    }
}

impl Solve for Advent {
    fn get_label(&self) -> &Label { &self.label }
    fn get_label_mut(&mut self) -> &mut Label { &mut self.label }

    fn add_record_from_line(&mut self, line: String) -> Result<(), std::num::ParseIntError> {
        let command = match line.split_whitespace().collect::<Vec<_>>()[..] {
            [cmd, arg0, arg1] => {
                let first_ch0 = arg0.chars().nth(0).unwrap();
                match cmd {
                    "cpy" => {
                        let first_ch1 = arg1.chars().nth(0).unwrap();
                        if self.register_keys.contains(&first_ch0) {
                            Command::new(Instruction::CPY, Some(first_ch0), Some(first_ch1), None, None)
                        } else {
                            Command::new(Instruction::CPY, None, Some(first_ch1), Some(arg0.parse::<isize>()?), None)
                        }
                    }
                    "jnz" => {
                        let offset = Some(arg1.parse::<isize>()?);
                        if self.register_keys.contains(&first_ch0) {
                            Command::new(Instruction::JNZ, Some(first_ch0), None, None, offset)
                        } else {
                            Command::new(Instruction::JNZ, None, None, Some(arg0.parse::<isize>()?), offset)
                        }
                    }
                    _ => unreachable!()
                }
            }
            [cmd, arg0] => {
                let register = Some(arg0.chars().nth(0).unwrap());
                match cmd {
                    "inc" => {
                        Command::new(Instruction::INC, None, register, None, None)
                    }
                    "dec" => {
                        Command::new(Instruction::DEC, None, register, None, None)
                    }
                    _ => unreachable!()
                }
            }
            _ => unreachable!()
        };
        self.program.push(command);
        Ok(())
    }

    fn info(&self) -> Result<(), String> {
        self.check_input(None)?;
        println!("Program length: {}", self.program.len());
        Ok(())
    }
    fn compute_part1_answer(&self, _test_mode: bool) -> Result<String, String> {
        self.check_input(Some(1))?;
        let mut registers: HashMap<char, isize> = HashMap::new();
        self.solve(&mut registers,  318020, 1)
    }
    fn compute_part2_answer(&self, _test_mode: bool) -> Result<String, String> {
        self.check_input(Some(2))?;
        let mut registers: HashMap<char, isize> = HashMap::new();
        registers.insert('c',1);
        self.solve(&mut registers,  9227674, 2)
    }
}