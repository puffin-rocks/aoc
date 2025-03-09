use std::collections::HashMap;
use itertools::enumerate;
use crate::utils::{assert_display, Label, Solve};

pub(crate) struct Advent {
    label: Label,
    memory: Vec<usize>
}
impl Default for Advent {
    fn default() -> Self{
        Self{
            label: Label::new(6, 2017),
            memory: Vec::new()
        }
    }
}

impl Advent {
    fn solve(&self,
             result_prd: usize,
             part: u8
    ) -> Result<String, String> {
        self.check_input(Some(part))?;
        let mut states: HashMap<String, usize> = HashMap::new();
        let mut n_steps = 0usize;
        states.insert(state(&self.memory), n_steps);
        let mut memory = self.memory.clone();
        let cycle_length: Option<usize>;
        loop{
            let mut m_max = *memory.iter().max().unwrap();
            let mut memory_next = memory.clone();
            let mut searching = true;
            for (e, m) in enumerate(memory.iter()).cycle(){
                if searching {
                    if *m == m_max {
                        memory_next[e] = 0;
                        searching = false;
                    }
                }
                else{
                    if m_max==0{
                        break;
                    }else{
                        memory_next[e]+=1;
                        m_max-=1;
                    }
                }
            }
            memory = memory_next;
            n_steps+=1;
            let s = state(&memory);
            if states.contains_key(&s){
                cycle_length = Some(n_steps-states.get(&s).unwrap());
                break;
            }
            states.insert(s, n_steps);
        }
        let (result, header) = if part==1{
            (n_steps, "Steps till cycle")
        }else{
            (cycle_length.unwrap(), "Cycle length")
        };
        assert_display(result, None, result_prd, header,false)
    }
}

impl Solve for Advent {

    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn add_record_from_line(&mut self, line: String) -> Result<(), std::num::ParseIntError>{
        self.memory = line.split_whitespace()
            .map(|x| x.parse::<usize>().expect("Cannot convert"))
            .collect::<Vec<_>>();
        Ok(())
    }

    fn info(&self) -> Result<(), String>{
        self.check_input(None)?;
        println!("Memory length: {}", self.memory.len());
        Ok(())
    }
    fn compute_part1_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.solve(5042, 1)
    }
    fn compute_part2_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.solve(1086, 2)
    }
}

fn state(input: &Vec<usize>)->String{
    input.iter().map(|x| format!("{}.", x)).collect::<String>()
}