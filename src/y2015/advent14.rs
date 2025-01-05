use std::cmp::min;
use std::collections::HashMap;
use crate::utils::{assert_display, Label, Solve};
use regex::Regex;

struct Reindeer{
    name: String,
    speed: usize,
    run_time: usize,
    rest_time: usize
}

impl Reindeer {
    fn new(name: String, speed: usize, run_time: usize, rest_time:usize) ->Self{
        Self{
            name,
            speed,
            run_time,
            rest_time
        }
    }

    fn cycle_len(&self) -> usize{
        self.run_time + self.rest_time
    }

    fn location(&self, t: usize) -> usize{
        let ct = self.cycle_len();
        let n_cycles = t / ct;
        let rem_time = min(t % ct, self.run_time);
        self.speed * (n_cycles * self.run_time + rem_time)
    }
}

pub(crate) struct Advent {
    label: Label,
    reindeers: Vec<Reindeer>
}
impl Default for Advent {
    fn default() -> Self{
        Self{
            label: Label::new(14, 2015),
            reindeers: Vec::new()
        }
    }
}

impl Solve for Advent {

    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn add_record_from_line(&mut self, line: String) -> Result<(), std::num::ParseIntError>{
        let re = Regex::new(r"(\w+) can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds.").unwrap();
        if let Some(captures) = re.captures(line.as_str()) {
            let name = String::from(captures.get(1).unwrap().as_str());
            let speed = captures.get(2).unwrap().as_str().parse::<usize>()?;
            let run_time = captures.get(3).unwrap().as_str().parse::<usize>()?;
            let rest_time = captures.get(4).unwrap().as_str().parse::<usize>()?;
            self.reindeers.push(Reindeer::new(name, speed, run_time, rest_time));
        } else {
            "invalid".parse::<i32>()?;
        }
        Ok(())
    }

    fn info(&self) -> Result<(), String>{
        self.check_input(None)?;
        println!("Number of reindeers: {}", self.reindeers.len());
        Ok(())
    }
    fn compute_part1_answer(&self, _test_mode: bool) -> Result<String, String>{
        let t: usize = 2503;
        let result = self.reindeers.iter().map(|r| {
            r.location(t)
        }).max().unwrap();
        assert_display(result, None, 2655, "Maximal distance", false)
    }
    fn compute_part2_answer(&self, _test_mode: bool) -> Result<String, String>{
        let mut scores: HashMap<&String, usize> = HashMap::new();
        for r in self.reindeers.iter(){
            scores.insert(&r.name, 0);
        }
        for t in 1..=2503 {
            let distances = self.reindeers.iter().map(|r| {
                (&r.name, r.location(t))
            }).collect::<Vec<_>>();
            let m = distances.iter().map(|(_, d)| d).max().unwrap();
            for (n, d) in distances.iter(){
                if d == m{
                    *scores.get_mut(n).unwrap()+=1;
                }
            }
        }
        let result = *scores.values().max().unwrap();
        assert_display(result, None, 1059, "Maximal score", false)
    }
}