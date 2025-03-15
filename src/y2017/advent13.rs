use std::collections::HashMap;
use crate::utils::{assert_display, Label, Solve};

pub(crate) struct Advent {
    label: Label,
    layers: HashMap<usize, usize>,
    max_layer: usize
}
impl Default for Advent {
    fn default() -> Self{
        Self{
            label: Label::new(13, 2017),
            layers: HashMap::new(),
            max_layer: 0
        }
    }
}

impl Advent{
    fn severity(&self)->usize{
        let mut severity = 0usize;
        for t in 0..=self.max_layer{
            if let Some(length)=self.layers.get(&t){
                if on_top(t, *length){
                    severity+=length*t;
                }
            }
        }
        severity
    }
    fn passed(&self, delay:usize)->bool{
        for i in 0..=self.max_layer{
            let t=i+delay;
            if let Some(length)=self.layers.get(&i){
                if on_top(t, *length){
                   return false
                }
            }
        }
        true
    }
}
impl Solve for Advent {

    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn add_record_from_line(&mut self, line: String) -> Result<(), std::num::ParseIntError>{
        if let Some((left, right)) = line.split_once(": "){
            self.layers.insert(left.parse::<usize>()?, right.parse::<usize>()?);
            self.max_layer = *self.layers.keys().max().unwrap();
        }
        Ok(())
    }

    fn info(&self) -> Result<(), String>{
        self.check_input(None)?;
        println!("Number of layers: {}", self.layers.len());
        Ok(())
    }
    fn compute_part1_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.check_input(Some(1))?;
        assert_display(self.severity(), None, 1624, "Severity", false)
    }
    fn compute_part2_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.check_input(Some(2))?;

        for delay in 1..= usize::MAX{
            if self.passed(delay){
                return assert_display(delay, None, 3923436, "Minimal delay", false)
            }
        }
        Err(String::from("No solution found"))
    }
}

fn on_top(time:usize, length:usize)->bool{
    if length == 1{
        true
    }else {
        time % ((length - 1) * 2) == 0
    }
}

