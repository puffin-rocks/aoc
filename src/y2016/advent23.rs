use std::collections::{HashMap, HashSet};
use crate::hashset;
use crate::utils::{assert_display, Label, Solve};

#[derive(Debug, Clone)]
enum Instruction {
    CPY,
    INC,
    DEC,
    JNZ,
    TGL
}

#[derive(Debug, Clone)]
struct Command {
    instruction: Instruction,
    register_source: Option<char>,
    register_target: Option<char>,
    value: Option<isize>,
    offset: Option<isize>,
    register_offset: Option<char>
}

impl Command {
    fn new(instruction: Instruction,
           register_source: Option<char>,
           register_target: Option<char>,
           value: Option<isize>,
           offset: Option<isize>,
           register_offset: Option<char>
    ) -> Self {
        Self {
            instruction,
            register_source,
            register_target,
            value,
            offset,
            register_offset
        }
    }

    fn run(&self, index: usize, registers: &mut HashMap<char, isize>) -> (usize, Option<usize>) {
        match self.instruction {
            Instruction::CPY | Instruction::JNZ | Instruction::TGL => {
                let value = if self.register_source.is_none() {
                    self.value.unwrap()
                } else {
                    let ch = self.register_source.unwrap();
                    *registers.get(&ch).unwrap_or(&0)
                };
                match self.instruction{
                    Instruction::CPY=>{
                        if self.offset.is_some(){
                        }
                        else if self.register_offset.is_some() {
                            registers.insert(self.register_offset.unwrap(), value);
                        }
                        else {
                            registers.insert(self.register_target.unwrap(), value);
                        }
                        (index + 1, None)
                    },
                    Instruction::JNZ => {
                        if value != 0 {
                            let offset = if self.register_offset.is_none() {
                                self.offset.unwrap()
                            } else {
                                let ch = self.register_offset.unwrap();
                                *registers.get(&ch).unwrap_or(&0)
                            };
                            ((index as isize + offset) as usize, None)
                        } else {
                            (index + 1, None)
                        }
                    },
                    Instruction::TGL => {
                        let toggle_value = index as isize + value;
                        if toggle_value>=0 {
                            (index + 1, Some(toggle_value as usize))
                        }else{
                            (index+1, None)
                        }
                    }
                    _ => (index + 1, None)
                }

            }
            Instruction::DEC => {
                *registers.entry(self.register_target.unwrap()).or_insert(0) -= 1;
                (index + 1, None)
            }
            Instruction::INC => {
                *registers.entry(self.register_target.unwrap()).or_insert(0) += 1;
                (index + 1, None)
            }
        }
    }
    fn toggle(&self) -> Self{
        let toggled_instruction = match &self.instruction{
            Instruction::INC =>{
                Instruction::DEC
            },
            Instruction::DEC | Instruction::TGL =>{
                Instruction::INC
            },
            Instruction::JNZ =>{
                Instruction::CPY
            },
            Instruction::CPY =>{
                Instruction::JNZ
            }
        };
        Command::new(toggled_instruction, self.register_source, self.register_target, self.value, self.offset, self.register_offset)
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
            label: Label::new(23, 2016),
            program: Vec::new(),
            register_keys: hashset!['a', 'b', 'c', 'd'],
        }
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
                            Command::new(Instruction::CPY, Some(first_ch0), Some(first_ch1), None, None, None)
                        } else {
                            Command::new(Instruction::CPY, None, Some(first_ch1), Some(arg0.parse::<isize>()?), None, None)
                        }
                    }
                    "jnz" => {
                        let first_ch1 = arg1.chars().nth(0).unwrap();
                        if self.register_keys.contains(&first_ch0) {
                            if self.register_keys.contains(&first_ch1) {
                                Command::new(Instruction::JNZ, Some(first_ch0), None, None, None, Some(first_ch1))
                            }
                            else{
                                Command::new(Instruction::JNZ, Some(first_ch0), None, None, Some(arg1.parse::<isize>()?), None)
                            }
                        } else {
                            if self.register_keys.contains(&first_ch1) {
                                Command::new(Instruction::JNZ, None, None, Some(arg0.parse::<isize>()?), None, Some(first_ch1))
                            } else {
                                Command::new(Instruction::JNZ, None, None, Some(arg0.parse::<isize>()?), Some(arg1.parse::<isize>()?), None)
                            }
                        }
                    }
                    _ => unreachable!()
                }
            }
            [cmd, arg0] => {
                let first_ch0 = arg0.chars().nth(0).unwrap();

                match cmd {
                    "inc" => {
                        Command::new(Instruction::INC, None, Some(first_ch0), None, None, None)
                    }
                    "dec" => {
                        Command::new(Instruction::DEC, None, Some(first_ch0), None, None, None)
                    }
                    "tgl" => {
                        if self.register_keys.contains(&first_ch0) {
                            Command::new(Instruction::TGL, Some(first_ch0), None, None, None, None)
                        }else{
                            Command::new(Instruction::TGL, None, None, Some(arg0.parse::<isize>()?), None, None)
                        }
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
        registers.insert('a', 7);

        let mut index = 0;
        let mut toggle:Option<usize>;
        let mut program = self.program.clone();
        while index < program.len() {
            (index, toggle) = program.get(index).unwrap().run(index, &mut registers);
            match toggle{
                None => {},
                Some(toggle_index)=>{
                    if toggle_index<program.len(){
                        program[toggle_index] = program[toggle_index].toggle();
                    }
                }
            }
        }
        assert_display(*registers.get(&'a').unwrap(), None, 11424, "Value in 'a'", false)
    }
    fn compute_part2_answer(&self, _test_mode: bool) -> Result<String, String> {
        self.check_input(Some(2))?;
        let mut registers: HashMap<char, isize> = HashMap::new();
        registers.insert('a', 12);

        let mut index = 0;
        let mut toggle:Option<usize>;
        let mut program = self.program.clone();
        while index < program.len() {
            (index, toggle) = program.get(index).unwrap().run(index, &mut registers);
            match toggle{
                None => {},
                Some(toggle_index)=>{
                    if toggle_index<program.len(){
                        program[toggle_index] = program[toggle_index].toggle();
                    }
                }
            }
            if index == 10{
                //shortcut
                let b = *registers.get(&'b').unwrap()-1;
                registers.insert('b', b);
                registers.insert('c', 2*b);
                index = 16;
            }
            if index == 17{
                //shortcut
                let a = *registers.get(&'a').unwrap();
                let b = *registers.get(&'b').unwrap();
                if toggle!=Some(18) {
                    registers.insert('a', a * b);
                    registers.insert('c', 0);
                    index = 9;
                }
            }
        }
        assert_display(*registers.get(&'a').unwrap(), None, 479007984, "Value in 'a'", false)
    }
}