use std::collections::HashMap;
use crate::geometry::{Direction, Point2D};
use crate::utils::{assert_display, Label, Solve};

pub(crate) struct Advent {
    label: Label,
    input: usize
}
impl Default for Advent {
    fn default() -> Self{
        Self{
            label: Label::new(3, 2017),
            input: 0
        }
    }
}

impl Solve for Advent {

    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn add_record_from_line(&mut self, line: String) -> Result<(), std::num::ParseIntError>{
        self.input = line.parse::<usize>()?;
        Ok(())
    }

    fn info(&self) -> Result<(), String>{
        self.check_input(None)?;
        println!("Input: {}", self.input);
        Ok(())
    }
    fn compute_part1_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.check_input(Some(1))?;
        let mut sum = 1;
        let mut step = 0;
        while sum<self.input{
            step+=1;
            sum+=step*8;
        }
        let distance = if step ==0{
            0
        }else {
            sum -= step * 8;
            let (x, y) = coordinates((self.input-sum) as isize, step as isize);
            (x.abs()+y.abs()) as usize
        };
        assert_display(distance, None, 552, "Distance", false)
    }
    fn compute_part2_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.check_input(Some(2))?;
        let mut grid: HashMap<(isize, isize), usize> = HashMap::new();
        let mut value = 1;
        let mut step: usize = 0;
        let mut key = (0isize,0isize);
        grid.insert((0,0), value);
        let mut cnt = 1isize;
        let dirs = [&Direction::base()[..], &Direction::diagonal()[..]].concat();
        while value<self.input{
            if key == (step as isize, step as isize){
                step+=1;
                cnt=0;
            }
            cnt+=1;
            key = coordinates(cnt, step as isize);
            value = 0;
            for d in &dirs{
                let neighbour = &Point2D::new(key.0, key.1)+d;
                if let Some(v) = grid.get(&(*neighbour.x(), *neighbour.y())){
                    value+=v;
                }
            }
            grid.insert(key,value);
        }
        assert_display(value, None, 330785, "First larger value", false)
    }
}

fn coordinates(diff: isize, step: isize) ->(isize, isize){
    let corner = (diff - 1) / (step * 2);
    let rem = (diff - 1) % (step * 2) + 1;
    match corner {
        0 => (step, step - rem),
        1 => (step - rem, -step),
        2 => (-step, -(step - rem)),
        3 => (-(step - rem), step),
        _ => unreachable!()
    }
}