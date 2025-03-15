use std::collections::HashMap;
use regex::Regex;
use crate::utils::{assert_display, Label, Solve};

pub(crate) struct Advent {
    label: Label,
    seeds: HashMap<String, usize>,
    multipliers: HashMap<String, usize>,
    criteria: HashMap<String, usize>
}
impl Default for Advent {
    fn default() -> Self{
        let mut multipliers: HashMap<String, usize> = HashMap::new();
        multipliers.insert(String::from("A"), 16807);
        multipliers.insert(String::from("B"), 48271);
        let mut criteria: HashMap<String, usize> = HashMap::new();
        criteria.insert(String::from("A"), 4);
        criteria.insert(String::from("B"), 8);
        Self{
            label: Label::new(15, 2017),
            seeds: HashMap::new(),
            multipliers,
            criteria
        }
    }
}

impl Solve for Advent {

    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn add_record_from_line(&mut self, line: String) -> Result<(), std::num::ParseIntError>{
        let re = Regex::new(r"Generator (\w+) starts with (\d+)").unwrap();
        if let Some(captures) = re.captures(line.as_str()) {
            let name = String::from(captures.get(1).unwrap().as_str());
            let seed = captures.get(2).unwrap().as_str().parse::<usize>()?;
            self.seeds.insert(name,seed);
        }
        Ok(())
    }

    fn info(&self) -> Result<(), String>{
        self.check_input(None)?;
        println!("Seeds: {:?}", self.seeds);
        Ok(())
    }
    fn compute_part1_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.check_input(Some(1))?;
        const N_PAIRS: usize = 40_000_000;
        let mut a = *self.seeds.get("A").unwrap();
        let m_a = *self.multipliers.get("A").unwrap();
        let mut b = *self.seeds.get("B").unwrap();
        let m_b = *self.multipliers.get("B").unwrap();
        let div = 2147483647usize;
        let mask = (1 << 16) - 1;
        let mut cnt = 0;
        for _ in 0..N_PAIRS{
            a = (a*m_a)%div;
            b = (b*m_b)%div;
            if a & mask == b & mask{
                cnt+=1;
            }
        }
        assert_display(cnt, None, 631, "Final count", false)
    }
    fn compute_part2_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.check_input(Some(2))?;
        const N_PAIRS: usize = 5_000_000;
        let mut a = *self.seeds.get("A").unwrap();
        let m_a = *self.multipliers.get("A").unwrap();
        let c_a = *self.criteria.get("A").unwrap();
        let mut b = *self.seeds.get("B").unwrap();
        let m_b = *self.multipliers.get("B").unwrap();
        let c_b = *self.criteria.get("B").unwrap();
        let mask = (1 << 16) - 1;
        let mut cnt = 0;
        for _ in 0..N_PAIRS{
            a = next_number(a, m_a, c_a);
            b = next_number(b, m_b, c_b);
            if a & mask == b & mask{
                cnt+=1;
            }
        }
        assert_display(cnt, None, 279, "Final count", false)
    }
}

fn next_number(n: usize, m: usize, c:usize) -> usize{
    let div = 2147483647usize;
    let mut n = (n*m)%div;
    while n%c > 0{
        n = (n*m)%div;
    }
    n
}