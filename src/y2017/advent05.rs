use crate::utils::{assert_display, Label, Solve};

pub(crate) struct Advent {
    label: Label,
    jumps: Vec<isize>
}
impl Default for Advent {
    fn default() -> Self{
        Self{
            label: Label::new(5, 2017),
            jumps: Vec::new()
        }
    }
}

impl Advent {
    fn solve(&self,
             oscillating: bool,
             result_prd: usize,
             part: u8
    ) -> Result<String, String> {
        self.check_input(Some(part))?;
        let mut jumps = self.jumps.clone();
        let n = jumps.len() as isize;
        let mut pos: isize = 0;
        let mut n_steps = 0;
        while pos>-1 && pos<n{
            let value = jumps[pos as usize];
            let increment = if oscillating && value>2 {
                -1
            }else{
                1
            };
            jumps[pos as usize]+=increment;
            pos+=value;
            n_steps+=1;
        }
        assert_display(n_steps, None, result_prd, "Number of steps", false)
    }
}

impl Solve for Advent {

    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn add_record_from_line(&mut self, line: String) -> Result<(), std::num::ParseIntError>{
        self.jumps.push(line.parse::<isize>()?);
        Ok(())
    }

    fn info(&self) -> Result<(), String>{
        self.check_input(None)?;
        println!("Number of instructions: {}", self.jumps.len());
        Ok(())
    }
    fn compute_part1_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.solve(false,358309,1)
    }
    fn compute_part2_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.solve(true,28178177,2)
    }
}