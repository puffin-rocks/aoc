use std::collections::BTreeSet;
use crate::utils::{assert_display, Label, Solve};

pub(crate) struct Advent {
    label: Label,
    blacklist: BTreeSet<(u32,u32)>
}
impl Default for Advent {
    fn default() -> Self{
        Self{
            label: Label::new(20, 2016),
            blacklist: BTreeSet::new()
        }
    }
}

impl Solve for Advent {

    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn add_record_from_line(&mut self, line: String) -> Result<(), std::num::ParseIntError>{
        if let Some((from, to)) = line.split_once("-"){
            self.blacklist.insert((from.parse::<u32>()?, to.parse::<u32>()?));
        }
        Ok(())
    }

    fn info(&self) -> Result<(), String>{
        self.check_input(None)?;
        println!("List length: {}", self.blacklist.len());
        Ok(())
    }
    fn compute_part1_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.check_input(Some(1))?;
        let mut candidate = 0u32;
        let mut result: Option<u32> = None;
        loop {
            for (f, t) in self.blacklist.iter() {
                if f<=&candidate && t>=&candidate{
                    candidate = t+1;
                    break;
                }
                if f>&candidate{
                    result = Some(candidate);
                    break;
                }
            }
            if result.is_some(){
                break;
            }
        }
        assert_display(result.unwrap(), None, 32259706, "Lowest allowed IP", false)
    }
    fn compute_part2_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.check_input(Some(2))?;
        let mut candidate = 0u32;
        let mut n_allowed = 0;
        let max_limit = self.blacklist.iter().map(|(_, t)| *t).max().unwrap();
        while candidate<max_limit {
            for (f, t) in self.blacklist.iter() {
                if f<=&candidate && t>=&candidate{
                    candidate = t+1;
                    break;
                }
                if f>&candidate{
                    n_allowed+=f-candidate;
                    if t < &u32::MAX {
                        candidate = t + 1;
                    }else{
                        candidate = u32::MAX;
                    }
                    break;
                }
            }
        }
        if candidate<u32::MAX {
            n_allowed += u32::MAX - (candidate+1);
        }else{
            if max_limit<u32::MAX{
                n_allowed +=1
            }
        }
        assert_display(n_allowed, None, 113, "Number of allowed IPs", false)
    }
}