use std::collections::{HashMap, HashSet};
use itertools::izip;
use crate::geometry::Point2D;
use crate::utils::{assert_display, Label, Solve};

#[derive(Debug, Clone, Hash, PartialEq)]
enum Action{
    TurnOn,
    TurnOff,
    Toggle
}

impl Eq for Action{}

pub(crate) struct Advent {
    label: Label,
    instructions: Vec<(Action, Point2D, Point2D)>,
    lines: Vec<String>
}
impl Default for Advent {
    fn default() -> Self{
        Self{
            label: Label::new(6, 2015),
            instructions: Vec::new(),
            lines: Vec::new()
        }
    }
}

impl Solve for Advent {

    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn add_record_from_line(&mut self, line: String) -> Result<(), std::num::ParseIntError>{
        if let Some((part1, p2)) = line.split_once(" through "){
            if let Some((action, p1)) = part1.rsplit_once(" "){
                let action = match action {
                    "turn on" => Action::TurnOn,
                    "turn off" => Action::TurnOff,
                    "toggle" => Action::Toggle,
                    _ => unreachable!()
                };
                if let (Some((x1,y1)), Some((x2,y2))) =
                    (p1.split_once(","), p2.split_once(",")){
                    let p1 = Point2D::new(x1.parse::<usize>()?, y1.parse::<usize>()?);
                    let p2 = Point2D::new(x2.parse::<usize>()?, y2.parse::<usize>()?);
                    self.instructions.push((action, p1, p2));
                }
            }
        }
        self.lines.push(line);
        Ok(())
    }

    fn info(&self) -> Result<(), String>{
        self.check_input(None)?;
        println!("Number of instructions: {}", self.instructions.len());
        Ok(())
    }

    fn compute_part1_answer(&self, _: bool) -> Result<String, String>{
        self.check_input(Some(1))?;
        let mut lights = [[false; 1000]; 1000];
        for (action, from, to) in self.instructions.iter() {
            for x in *from.x() as usize..=*to.x() as usize{
                for y in *from.y() as usize..=*to.y() as usize{
                    match action {
                        Action::TurnOn => {
                            lights[x][y]=true;
                        },
                        Action::TurnOff => {
                            lights[x][y]=false;
                        },
                        Action::Toggle => {
                            lights[x][y]=!lights[x][y]
                        }
                    };
                }
            }

        }
        let result = lights.iter()
            .map(|x|
                {x.iter().map(|y| {if *y==true {1} else{0}}).sum::<usize>()}).sum::<usize>();
        assert_display(result, None, 569999, "Number of lights on", false)
    }
    //
    // fn compute_part2_answer(&self,  _: bool) -> Result<String, String>{
    //     self.check_input(Some(2))?;
    //     Err(String::from("Part 2 not implemented yet"))
    // }
}