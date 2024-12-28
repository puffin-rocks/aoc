use std::collections::HashSet;
use crate::geometry::{Direction, Point2D};
use crate::utils::{assert_display, Label, Solve};

pub(crate) struct Advent {
    label: Label,
    instructions: String
}
impl Default for Advent {
    fn default() -> Self{
        Self{
            label: Label::new(3, 2015),
            instructions: String::new()
        }
    }
}

impl Solve for Advent {

    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn add_record_from_line(&mut self, line: String) -> Result<(), std::num::ParseIntError>{
        self.instructions = line;
        Ok(())
    }

    fn info(&self) -> Result<(), String>{
        self.check_input(None)?;
        println!("Instruction length: {}", self.instructions.len());
        Ok(())
    }

    fn compute_part1_answer(&self, _: bool) -> Result<String, String>{
        self.check_input(Some(1))?;
        let mut visited: HashSet<Point2D> = HashSet::new();
        let mut p = Point2D::new(0,0);
        visited.insert(p.clone());
        for ch in self.instructions.chars(){
            p = &p + &Direction::from_char(&ch);
            visited.insert(p.clone());
        }
        assert_display(visited.len(), None, 2565, "Number of houses", false)
    }

    fn compute_part2_answer(&self,  _: bool) -> Result<String, String>{
        self.check_input(Some(2))?;
        let mut visited: HashSet<Point2D> = HashSet::new();
        let mut p1 = Point2D::new(0,0);
        let mut p2 = Point2D::new(0,0);
        visited.insert(p1.clone());
        for (i, ch) in self.instructions.chars().enumerate(){
            if i%2 == 0 {
                p1 = &p1 + &Direction::from_char(&ch);
                visited.insert(p1.clone());
            }else{
                p2 = &p2 + &Direction::from_char(&ch);
                visited.insert(p2.clone());
            }
        }
        assert_display(visited.len(), None, 2639, "Number of houses", false)
    }
}