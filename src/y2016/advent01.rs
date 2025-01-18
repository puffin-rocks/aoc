use std::collections::HashSet;
use crate::geometry::{Direction, Point2D, Vector};
use crate::utils::{assert_display, Label, Solve};

pub(crate) struct Advent {
    label: Label,
    sequence: Vec<(char, usize)>
}
impl Default for Advent {
    fn default() -> Self{
        Self{
            label: Label::new(1, 2016),
            sequence: Vec::new()
        }
    }
}

impl Advent {
    fn solve(&self,
             first_visited: bool,
             result_prd: usize,
             part: u8
    ) -> Result<String, String> {
        self.check_input(Some(part))?;
        let mut position = Point2D::new(0,0);
        let mut direction = Direction::Down;
        let mut vector: Vector;
        let mut visited: HashSet<Point2D> = HashSet::new();
        visited.insert(position.clone());
        let mut taxicab_distance: Option<usize> = None;
        for (side, step) in self.sequence.iter(){
            direction = match side{
                'R' => match direction{
                    Direction::Down => Direction::Right,
                    Direction::Right => Direction::Up,
                    Direction::Up => Direction::Left,
                    Direction::Left => Direction::Down,
                    _ => unreachable!()
                },
                'L' => match direction{
                    Direction::Down => Direction::Left,
                    Direction::Right => Direction::Down,
                    Direction::Up => Direction::Right,
                    Direction::Left => Direction::Up,
                    _ => unreachable!()
                }
                _ => unreachable!()
            };
            vector = Vector::new(direction, position);
            if first_visited {
                for s in 0..*step {
                    position = vector.get_point(s + 1);
                    if visited.contains(&position) {
                        taxicab_distance = Some((position.x().abs() + position.y().abs()) as usize);
                        break;
                    } else {
                        visited.insert(position.clone());
                    }
                }
            }
            else{
                position = vector.get_point(*step);
            }
            if taxicab_distance.is_some(){
                break;
            }
        };
        if !first_visited {
            taxicab_distance = Some((position.x().abs() + position.y().abs()) as usize);
        }
        match taxicab_distance {
            None => Err(format!("No solution for part {} found", part)),
            Some(taxicab_distance) => assert_display(taxicab_distance, None, result_prd, "Distance", false)
        }
    }
}

impl Solve for Advent {

    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn add_record_from_line(&mut self, line: String) -> Result<(), std::num::ParseIntError>{
        for x in  line.split(", "){
            self.sequence.push((x.chars().nth(0).unwrap(), *(&x[1..].parse::<usize>()?)));
        }
        Ok(())
    }

    fn info(&self) -> Result<(), String>{
        self.check_input(None)?;
        println!("Sequence length: {}", self.sequence.len());
        Ok(())
    }
    fn compute_part1_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.solve(false, 239, 1)
    }
    fn compute_part2_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.solve(true, 141, 2)
    }

}
