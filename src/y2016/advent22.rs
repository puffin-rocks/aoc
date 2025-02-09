use std::collections::HashMap;
use regex::Regex;
use crate::utils::{assert_display, Label, Solve};

pub(crate) struct Advent {
    label: Label,
    grid: HashMap<(usize,usize),(usize,usize)>
}
impl Default for Advent {
    fn default() -> Self{
        Self{
            label: Label::new(22, 2016),
            grid: HashMap::new()
        }
    }
}

impl Solve for Advent {

    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn add_record_from_line(&mut self, line: String) -> Result<(), std::num::ParseIntError>{
        //Filesystem              Size  Used  Avail  Use%
        // if line.starts_with("root") | line.starts_with("Filesystem"){
        //     return Ok(())
        // }

        let re = Regex::new(r"/dev/grid/node-x(\d+)-y(\d+)\s+(\d+)T\s+(\d+)T\s+(\d+)T\s+(\d+)%").unwrap();
        if let Some(captures) = re.captures(line.as_str()){
            let x = captures.get(1).unwrap().as_str().parse::<usize>()?;
            let y = captures.get(2).unwrap().as_str().parse::<usize>()?;
            let used = captures.get(4).unwrap().as_str().parse::<usize>()?;
            let avail = captures.get(5).unwrap().as_str().parse::<usize>()?;
            self.grid.insert((x,y), (used, avail));
        }
        Ok(())
    }

    fn info(&self) -> Result<(), String>{
        self.check_input(None)?;
        println!("Number of nodes: {}",self.grid.len());
        Ok(())
    }
    fn compute_part1_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.check_input(Some(1))?;
        let n = self.grid.iter()
            .flat_map(|(k1, v1)| {
                self.grid.iter().filter(move |(k2, v2)| {
                    k1 != *k2 && v1.0 > 0 && v1.0 <= v2.1
                })
            }).count();
        assert_display(n, None, 903, "Number of viable pairs", false)
    }
    fn compute_part2_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.check_input(Some(2))?;
        Err(String::from("Not implemented"))
    }
}