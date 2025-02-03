extern crate crypto;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};
use crypto::md5::Md5;
use crypto::digest::Digest;
use itertools::izip;
use crate::geometry::{Direction, Point2D};
use crate::hashset;
use crate::utils::{assert_display, Label, Solve};

pub(crate) struct Advent {
    label: Label,
    passcode: String
}
impl Default for Advent {
    fn default() -> Self{
        Self{
            label: Label::new(17, 2016),
            passcode: String::new()
        }
    }
}

impl Advent{
    fn compute_path_hash(&self, hasher: &mut Md5, path: &String) -> String {
        hasher.input(format!("{}{}", self.passcode, path).as_bytes());
        let mut output = [0; 16];
        hasher.result(&mut output);
        hasher.reset();
        hex::encode(output)
    }
}

impl Solve for Advent {

    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn add_record_from_line(&mut self, line: String) -> Result<(), std::num::ParseIntError>{
        self.passcode = line;
        Ok(())
    }

    fn info(&self) -> Result<(), String>{
        self.check_input(None)?;
        println!("Passcode: {}", self.passcode);
        Ok(())
    }
    fn compute_part1_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.check_input(Some(1))?;
        let mut visited: HashSet<String> = HashSet::new();
        let mut hasher = Md5::new();

        let target = Point2D::new(3,3);
        let mut queue: BinaryHeap<Reverse<(usize,Point2D, String)>> = BinaryHeap::new();
        queue.push(Reverse((0, Point2D::new(0,0), String::new())));
        let mut result: Option<String> = None;
        while let Some(Reverse((path_length, curr_point, path))) = queue.pop(){
            if curr_point == target{
                result = Some(path);
                break;
            }
            for d in hash2dir(self.compute_path_hash(&mut hasher, &path)){
                let next_point = &curr_point+&d;
                let mut next_path = path.clone();
                next_path.push(dir2char(&d));
                if is_valid(&next_point) && !visited.contains(&next_path){
                    visited.insert(next_path.clone());
                    queue.push(Reverse((path_length+1, next_point, next_path)));
                }
            }
        }
        match result{
            Some(path)=>assert_display(path, None, String::from("DRDRULRDRD"), "Shortest path", false), //
            None => Err(String::from("No solution found"))
        }
    }
    fn compute_part2_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.check_input(Some(2))?;
        let mut hasher = Md5::new();
        let target = Point2D::new(3,3);
        let mut queue: BinaryHeap<(usize,Point2D, String)> = BinaryHeap::new();
        queue.push((0, Point2D::new(0,0), String::new()));
        let mut result: Option<usize> = None;
        while let Some((path_length, curr_point, path)) = queue.pop() {
            if curr_point == target {
                match result {
                    None => result = Some(path_length),
                    Some(v)=>{
                        if v<path_length{
                            result = Some(path_length);
                        }
                    }
                }
                continue;
            }
            for d in hash2dir(self.compute_path_hash(&mut hasher, &path)) {
                let next_point = &curr_point + &d;
                let mut next_path = path.clone();
                next_path.push(dir2char(&d));
                if is_valid(&next_point){
                    queue.push((path_length + 1, next_point, next_path));
                }
            }
        }
        match result{
            Some(path_length)=>assert_display(path_length, None, 384, "Longest path", false), //
            None => Err(String::from("No solution found"))
        }
    }
}

fn is_valid(point: &Point2D)->bool{
    point.x()<&4 && point.y()<&4 && point.x()>&-1 && point.y()>&-1
}

fn dir2char(dir: &Direction) -> char{
    match dir{
        Direction::Down => 'U',
        Direction::Up => 'D',
        Direction::Right => 'R',
        Direction::Left => 'L',
        _ => unreachable!()
    }
}

fn hash2dir(hash: String)->HashSet<Direction>{
    let values = hash.chars().take(4).collect::<String>();
    let open = hashset!['b','c','d','e','f'];
    let mut result: HashSet<Direction> = HashSet::new();
    for (v, d ) in izip!(values.chars(), [Direction::Down, Direction::Up, Direction::Left, Direction::Right]){
        if open.contains(&v){
            result.insert(d);
        }
    }
    result
}