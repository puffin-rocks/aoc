use crate::geometry::{Direction, Point2D};
use crate::utils::{assert_display, Label, Solve};

pub(crate) struct Advent {
    label: Label,
    path: Vec<String>
}
impl Default for Advent {
    fn default() -> Self{
        Self{
            label: Label::new(11, 2017),
            path: Vec::new()
        }
    }
}

impl Solve for Advent {

    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn add_record_from_line(&mut self, line: String) -> Result<(), std::num::ParseIntError>{
        self.path = line.split(",").map(|x| String::from(x)).collect();
        Ok(())
    }

    fn info(&self) -> Result<(), String>{
        self.check_input(None)?;
        println!("Number of steps: {}", self.path.len());
        Ok(())
    }
    fn compute_part1_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.check_input(Some(1))?;
        let mut p = Point2D::new(0, 0);
        for step in self.path.iter(){
            p = &p+ &step2dir(step);
        }
        assert_display(shortest_path(&p), None, 720, "Fewest number of steps", false)
    }
    fn compute_part2_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.check_input(Some(2))?;
        let mut p = Point2D::new(0, 0);
        let mut furthest_distance = 0usize;
        for step in self.path.iter(){
            p = &p+ &step2dir(step);
            let d = shortest_path(&p);
            if d>furthest_distance{
                furthest_distance = d;
            }
        }
        assert_display(furthest_distance, None, 1485, "Furthest distance", false)
    }
}

fn step2dir(step: &String) -> Direction{
    match step.as_str(){
        "s" => Direction::ToPoint(Point2D::new(0,2)),
        "sw" => Direction::ToPoint(Point2D::new(-2, 1)),
        "se" => Direction::ToPoint(Point2D::new(2, 1)),
        "n" => Direction::ToPoint(Point2D::new(0,-2)),
        "nw" => Direction::ToPoint(Point2D::new(-2, -1)),
        "ne" => Direction::ToPoint(Point2D::new(2, -1)),
        _ => unreachable!()
    }
}

fn shortest_path(p: &Point2D) -> usize{
    let n_diag = p.x().abs()/2;
    let n_straight = (p.y().abs()-n_diag).abs()/2;
    (n_diag + n_straight) as usize
}