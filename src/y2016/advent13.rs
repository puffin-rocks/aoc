use std::collections::HashSet;
use crate::geometry::{Direction, Point2D};
use crate::hashset;
use crate::utils::{assert_display, Label, Solve};

pub(crate) struct Advent {
    label: Label,
    favorite_number: usize
}
impl Default for Advent {
    fn default() -> Self{
        Self{
            label: Label::new(13, 2016),
            favorite_number: 0
        }
    }
}

impl Advent{
    fn is_open(&self, x: &isize, y: &isize) -> bool {
        if x<&0 || y<&0 {
            false
        }else {
            format!("{:0b}", (*x * (*x + 3) + *y * (2 * *x + 1 + *y)) as usize + self.favorite_number)
                .chars()
                .filter(|x| x == &'1').count() % 2 == 0
        }
    }
}

impl Solve for Advent {

    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn add_record_from_line(&mut self, line: String) -> Result<(), std::num::ParseIntError>{
        self.favorite_number = line.parse::<usize>()?;
        Ok(())
    }

    fn info(&self) -> Result<(), String>{
        self.check_input(None)?;
        println!("Favorite number {}", self.favorite_number);
        Ok(())
    }
    fn compute_part1_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.check_input(Some(1))?;

        let start = Point2D::new(1,1);
        let finish = Point2D::new(31, 39);
        assert_eq!(self.is_open(start.x(), start.y()), true);
        assert_eq!(self.is_open(finish.x(), finish.y()), true);

        let mut queue = hashset!(start);
        let mut visited: HashSet<Point2D> = HashSet::new();
        let mut curr_step = 0;
        let mut result: Option<usize> = None;
        loop{
            let mut next_queue: HashSet<Point2D> = HashSet::new();
            for p in queue.iter(){
                for d in Direction::base(){
                    let next_p = p+&d;
                    if next_p==finish{
                        result = Some(curr_step+1);
                        break;
                    }
                    if !visited.contains(&next_p) && self.is_open(next_p.x(), next_p.y()){
                        next_queue.insert(next_p);
                        visited.insert(next_p);
                    }
                }
                if result.is_some(){
                    break;
                }
            }
            if result.is_some(){
                break;
            }
            queue = next_queue;
            if queue.is_empty(){
                break;
            }
            curr_step+=1;
        }
        match result{
            Some(result) => assert_display(result, None, 86, "Fewest number of steps", false),
            None=>Err(String::from("No solution found"))
        }
    }
    fn compute_part2_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.check_input(Some(2))?;

        let start = Point2D::new(1,1);
        assert_eq!(self.is_open(start.x(), start.y()), true);

        let mut queue = hashset!(start);
        let mut visited: HashSet<Point2D> = HashSet::new();
        let mut curr_step = 0;
        loop{
            if curr_step==50{
                break;
            }
            let mut next_queue: HashSet<Point2D> = HashSet::new();
            for p in queue.iter(){
                for d in Direction::base(){
                    let next_p = p+&d;
                    if !visited.contains(&next_p) && self.is_open(next_p.x(), next_p.y()){
                        next_queue.insert(next_p);
                        visited.insert(next_p);
                    }
                }
            }
            queue = next_queue;
            if queue.is_empty(){
                break;
            }
            curr_step+=1;
        }
        assert_display(visited.len(), None, 127, "Number of locations", false)
    }
}
