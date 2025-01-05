use std::collections::HashSet;
use crate::hashset;
use crate::utils::{assert_display, Label, Solve};

pub(crate) struct Advent {
    label: Label,
    containers: Vec<usize>
}
impl Default for Advent {
    fn default() -> Self{
        Self{
            label: Label::new(17, 2015),
            containers: Vec::new()
        }
    }
}

impl Advent {
    fn solve(&self,
             only_shortest: bool,
             result_prd: usize,
             part: u8
    ) -> Result<String, String> {
        self.check_input(Some(part))?;
        let assortment = (0..self.containers.len()).collect::<Vec<_>>();
        let mut stack:HashSet<(usize, Vec<usize>, Vec<usize>)> = hashset!((150usize, assortment, Vec::new()));
        let mut combinations: HashSet<Vec<usize>> = HashSet::new();
        loop{
            let mut next_stack: HashSet<(usize, Vec<usize>, Vec<usize>)> = HashSet::new();
            for (capacity, sortiment, current) in stack.iter(){
                for (i, c) in sortiment.iter().enumerate(){

                    let s = &self.containers[*c];
                    if capacity<s{
                        continue;
                    }
                    else if capacity>s{
                        let mut assortment = sortiment.clone();
                        assortment.remove(i);
                        let capacity = capacity - s;
                        assortment = assortment.iter().filter(|x| self.containers[**x] <= capacity).cloned().collect();

                        let mut result = current.clone();
                        result.push(*c);
                        result.sort_unstable();
                        next_stack.insert((capacity, assortment, result));
                    }else{
                        let mut result = current.clone();
                        result.push(*c);
                        result.sort_unstable();
                        combinations.insert(result);
                    }
                }
            }
            stack = next_stack;
            if stack.is_empty(){
                break;
            }
        }
        let n = if only_shortest {
            let min_length = combinations.iter().map(|c| c.len()).min().unwrap();
            combinations.iter().filter(|c| c.len() == min_length).count()
        }else{
            combinations.len()
        };
        assert_display(n, None, result_prd, "Number of combinations", false)
    }
}

impl Solve for Advent {

    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn add_record_from_line(&mut self, line: String) -> Result<(), std::num::ParseIntError>{
        let v = line.parse::<usize>()?;
        self.containers.push(v);
        Ok(())
    }

    fn info(&self) -> Result<(), String>{
        self.check_input(None)?;
        println!("Number of containers: {}", self.containers.len());
        Ok(())
    }
    fn compute_part1_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.solve(false,1638, 1)
    }
    fn compute_part2_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.solve(true,17, 2)
    }
}
