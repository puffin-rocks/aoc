use std::collections::HashMap;
use crate::utils::{assert_display, Label, Solve};

#[derive(Debug, Clone)]
enum Instruction{
    HLF,
    TPL,
    INC,
    JMP,
    JIE,
    JIO
}

#[derive(Debug, Clone)]
struct Command{
    instruction: Instruction,
    register: Option<char>,
    offset: Option<isize>
}

impl Command{
    fn new(instruction: Instruction, register: Option<char>, offset: Option<isize>) ->Self{
        Self{
            instruction,
            register,
            offset
        }
    }
    fn run(&self, index: usize, registers: &mut HashMap<char, usize>) -> usize{
        match self.instruction{
            Instruction::HLF =>{*registers.entry(self.register.unwrap()).or_insert(0)/=2;
                index+1},
            Instruction::TPL =>{*registers.entry(self.register.unwrap()).or_insert(0)*=3;
                index+1},
            Instruction::INC =>{*registers.entry(self.register.unwrap()).or_insert(0)+=1;
                index+1},
            Instruction::JMP =>{ (index as isize + self.offset.unwrap()) as usize},
            Instruction::JIE if registers.get(&self.register.unwrap()).unwrap()%2 == 0 =>{ (index as isize + self.offset.unwrap()) as usize},
            Instruction::JIO if registers.get(&self.register.unwrap()) == Some(&1) =>{ (index as isize + self.offset.unwrap()) as usize},
            _ => index+1
        }
    }
}

pub(crate) struct Advent {
    label: Label,
    program: Vec<Command>
}
impl Default for Advent {
    fn default() -> Self{
        Self{
            label: Label::new(23, 2015),
            program: Vec::new()
        }
    }
}

impl Advent {
    fn solve(&self,
             registers: &mut HashMap<char,usize>,
             result_prd: usize,
             part: u8
    ) -> Result<String, String> {
        self.check_input(Some(part))?;
        let mut index = 0;
        while index<self.program.len(){
            index = self.program.get(index).unwrap().run(index, registers);
        }
        assert_display(*registers.get(&'b').unwrap(), None, result_prd, "Value in 'b'", false)
    }
}

impl Solve for Advent {

    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn add_record_from_line(&mut self, line: String) -> Result<(), std::num::ParseIntError>{
        let (instr_reg, offset) =match line.split_once(", "){
            Some((instr_reg, offset)) =>{ (instr_reg, Some(offset.parse::<isize>()?))},
            None =>{ (line.as_str(), None)}
        };
        if let Some((instr, reg_or_offset)) = instr_reg.split_once(" "){
            let register = reg_or_offset.chars().nth(0);
            let cmd = match instr{
                "jmp" => Command::new(Instruction::JMP, None, Some(reg_or_offset.parse::<isize>()?)),
                "hlf" => Command::new(Instruction::HLF, register, offset),
                "tpl" => Command::new(Instruction::TPL, register, offset),
                "inc" => Command::new(Instruction::INC, register, offset),
                "jie" => Command::new(Instruction::JIE, register, offset),
                "jio" => Command::new(Instruction::JIO, register, offset),
                _ => unreachable!()
            };
            self.program.push(cmd);
        }
        Ok(())
    }

    fn info(&self) -> Result<(), String>{
        self.check_input(None)?;
        println!("Program length: {}", self.program.len());
        Ok(())
    }
    fn compute_part1_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.check_input(Some(1))?;
        let mut registers: HashMap<char, usize> = HashMap::new();
        self.solve(&mut registers, 184, 1)
    }
    fn compute_part2_answer(&self, _test_mode: bool) -> Result<String, String>{
        let mut registers: HashMap<char, usize> = HashMap::new();
        registers.insert('a', 1);
        self.solve(&mut registers, 231, 2)
    }
}