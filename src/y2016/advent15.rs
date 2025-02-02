use std::collections::HashMap;
use regex::Regex;
use crate::utils::{assert_display, Label, Solve};

pub(crate) struct Advent {
    label: Label,
    discs: HashMap<usize, (usize,usize)>
}
impl Default for Advent {
    fn default() -> Self{
        Self{
            label: Label::new(15, 2016),
            discs: HashMap::new()
        }
    }
}

impl Advent {
    fn solve(&self,
             discs: &HashMap<usize, (usize,usize)>,
             result_prd: usize,
             part: u8
    ) -> Result<String, String> {
        self.check_input(Some(part))?;
        let max_size = discs.iter().map(|(_, x)| x.0).max().unwrap();
        let index_max = discs.iter().filter_map(|(k, x)| if x.0==max_size {Some(k)} else {None}).collect::<Vec<_>>()[0];
        let mut result: Option<usize> = None;
        let position_shifted = discs.get(index_max).unwrap().1+*index_max;
        for c in 0usize..usize::MAX{
            if max_size*c>=position_shifted{
                let delay = max_size*c-position_shifted;
                let mut delay_fits = true;
                for (k, x) in discs.iter(){
                    if k==index_max{
                        continue;
                    }
                    if (x.1+delay+k)%x.0!=0{
                        delay_fits = false;
                        break;
                    }
                }
                if delay_fits{
                    result = Some(delay);
                    break;
                }
            }
        }
        match result{
            Some(delay) => assert_display(delay, None, result_prd, "Earliest start", false),
            None => Err(String::from("Not solution found"))
        }
    }
}

impl Solve for Advent {

    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn add_record_from_line(&mut self, line: String) -> Result<(), std::num::ParseIntError>{
        let re = Regex::new(r"Disc #(\d+) has (\d+) positions; at time=0, it is at position (\d+).").unwrap();
        if let Some(captures) = re.captures(line.as_str()) {
            let index = captures.get(1).unwrap().as_str().parse::<usize>()?;
            let size = captures.get(2).unwrap().as_str().parse::<usize>()?;
            let position = captures.get(3).unwrap().as_str().parse::<usize>()?;
            self.discs.insert(index, (size, position));
        }
        Ok(())
    }

    fn info(&self) -> Result<(), String>{
        self.check_input(None)?;
        println!("Number of discs: {}", self.discs.len());
        Ok(())
    }
    fn compute_part1_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.solve(&self.discs, 148737,1)
    }
    fn compute_part2_answer(&self, _test_mode: bool) -> Result<String, String>{
        let next_key = self.discs.iter().map(|(k, _)| k).max().unwrap()+1;
        let mut discs = self.discs.clone();
        discs.insert(next_key,(11, 0));
        self.solve(&discs, 2353212,2)
    }
}