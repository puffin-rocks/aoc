use crate::utils::{assert_display, Label, Solve};

pub(crate) struct Advent {
    label: Label,
    directions: String
}
impl Default for Advent {
    fn default() -> Self{
        Self{
            label: Label::new(1, 2015),
            directions: String::new()
        }
    }
}

impl Solve for Advent {

    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn add_record_from_line(&mut self, line: String) -> Result<(), std::num::ParseIntError>{
        self.directions = line;
        Ok(())
    }

    fn info(&self) -> Result<(), String>{
        self.check_input(None)?;
        println!("Number of directions: {}", self.directions.len());
        Ok(())
    }

    fn compute_part1_answer(&self, _: bool) -> Result<String, String>{
        self.check_input(Some(1))?;
        let m: usize = self.directions.chars().filter(|ch| ch==&'(').collect::<Vec<_>>().len();
        let floor = m-(self.directions.len()-m);
        assert_display(floor, None, 74, "Floor number", false)
    }

    fn compute_part2_answer(&self,  _: bool) -> Result<String, String>{
        self.check_input(Some(2))?;
        let mut floor = 0;
        let mut position: Option<usize> = None;
        for (i,ch) in self.directions.chars().enumerate(){
            match ch{
                '(' => floor+=1,
                ')' => floor-=1,
                _ => unreachable!()
            }
            if floor<0{
                position = Some(i+1);
                break;
            }
        };
        match position{
            Some(p)=> assert_display(p, None, 1795,
                                     "First position to enter basement", false),
            None => Err(String::from("No solution found"))
        }
    }
}