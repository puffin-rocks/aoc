use crate::utils::{assert_display, Label, Solve};

pub(crate) struct Advent {
    label: Label,
    step: usize
}
impl Default for Advent {
    fn default() -> Self{
        Self{
            label: Label::new(17, 2017),
            step: 0
        }
    }
}

impl Solve for Advent {

    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn add_record_from_line(&mut self, line: String) -> Result<(), std::num::ParseIntError>{
        self.step = line.parse::<usize>()?;
        Ok(())
    }

    fn info(&self) -> Result<(), String>{
        self.check_input(None)?;
        println!("Step length: {}", self.step);
        Ok(())
    }
    fn compute_part1_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.check_input(Some(1))?;
        const N:usize = 2017;
        let mut i = 0usize;
        let mut buffer = vec![i];
        let mut position = 0usize;
        while i<N {
            i+=1;
            position = (position + self.step) % buffer.len() + 1;
            buffer.insert(position, i);
        }
        let value_after = if position == buffer.len()-1{
            buffer[0]
        }else{
            buffer[position+1]
        };
        assert_display(value_after, None, 1971, "Value after 2017", false)
    }
    fn compute_part2_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.check_input(Some(2))?;
        const N:usize = 50_000_000;
        let mut i = 0usize;
        let mut position = 0usize;
        let mut loc_zero = 0usize;
        let mut after_zero = 0usize;
        while i<N {
            i+=1;
            position = (position + self.step) % i + 1;
            if loc_zero==i-1{
                if position==0{
                    after_zero = i;
                    loc_zero = i;
                }
            }else{
                if position<=loc_zero{
                    loc_zero+=1;
                }else if position == loc_zero+1{
                    after_zero = i;
                }
            }
        }
        assert_display(after_zero, None, 17202899, "Value after 0", false)
    }
}