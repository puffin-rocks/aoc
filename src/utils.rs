use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;
use crate::geometry::{Canvas, CanvasAsync};

#[macro_export]
macro_rules! hashset{
    ( $( $x: expr ),* $(,)? ) => {
        {
            let mut set = std::collections::HashSet::new();
            $(
            set.insert($x);
            )*
            let _ = &mut set;
            set
        }
    };
}

pub(crate) const PUZZLE_INPUT: &str  = "input/";

pub (crate) struct Label {
    pub(crate) day: u8,
    pub (crate) year: u16,
    pub(crate) has_input: bool
}


impl Label {
    pub fn new(day: u8, year: u16) -> Self{
        Self{
            day,
            year,
            has_input: false
        }
    }

    pub fn get_puzzle_input_path(&self, test_mode: bool) -> String{
        if test_mode {
            PUZZLE_INPUT.to_owned() + format!("{}/day{:0>2}_test.txt", self.year, self.day).as_str()
        }
        else{
            PUZZLE_INPUT.to_owned() + format!("{}/day{:0>2}.txt", self.year, self.day).as_str()
        }
    }
}

pub(crate) trait Solve
{
    fn get_label(&self) -> &Label;
    fn get_label_mut(&mut self) -> &mut Label;

    fn get_canvas_mut(&mut self) -> Option<&mut Canvas>{
        None
    }

    fn get_canvas_async_mut(&mut self) -> Option<&mut CanvasAsync>{
        None
    }

    fn apply_bruteforce(&mut self){}

    fn check_input(&self, part: Option<u8>) -> Result<(), String> {
        if self.get_label().has_input {
            Ok(())
        }
        else
        {
            match part{
                Some(part) => Err(no_solution_message(part)),
                None => Err(String::from("Advent is missing input"))
            }
        }
    }
    fn add_record_from_line(&mut self, line : String) -> Result<(), std::num::ParseIntError> {
        match self.get_canvas_mut(){
            None => {
                match self.get_canvas_async_mut(){
                    None => {
                        "invalid".parse::<i32>()?;
                    },
                    Some(canvas) => {
                        canvas.add_row(line.chars().collect());
                    }
                };
            },
            Some(canvas) => {
                canvas.add_row(line.chars().collect());
            }
        };

        Ok(())
    }

    fn read_input(&mut self, test_mode: bool) -> Result<(), std::num::ParseIntError>{
        let filename = self.get_label().get_puzzle_input_path(test_mode);

        if let Ok(lines) = read_lines(filename) {
            for line in lines.flatten() {
                self.add_record_from_line(line)?;
            }
            self.get_label_mut().has_input = true;
        }
        Ok(())
    }

    fn info(&self) -> Result<(), String> {
        Err(String::from("Advent is missing input"))
    }
    fn compute_part1_answer(&self, _test_mode: bool) -> Result<String, String>{
        Err(no_solution_message(1))
    }
    fn compute_part2_answer(&self, _test_mode: bool) -> Result<String, String>{
        Err(no_solution_message(2))
    }
}

//the 'static lifetime is a special lifetime that signifies the entire duration of the program.
pub fn add_default_to_collection<T:Default+Solve+ 'static>(collection: &mut HashMap<u8, Box<dyn Solve>>) {
    let a = T::default();
    collection.insert(a.get_label().day, Box::new(a));
}

pub fn no_solution_message(part: u8) -> String{
    format!("Part {} not solved", part)
}

pub fn assert_display<T: Debug+Display+PartialEq>(result: T,
         result_test: Option<T>,
         result_prd: T,
         header: &str,
         test_mode: bool
) -> Result<String, String>{
    match result_test {
        Some(result_test) => {
            assert_eq!(result, match test_mode{
                true =>  result_test,
                false => result_prd
            });
        },
        None => {
            match test_mode{
                true =>  {return Err(String::from("Test mode not implemented"));},
                false => assert_eq!(result, result_prd)
            }
        }
    };
    Ok(format!("{}: {}", header, result))
}

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn line2vec_i32(line: String) -> Result<Vec<i32>, std::num::ParseIntError> {
    let mut v: Vec<i32> = Vec::new();
    for p in line.split_whitespace() {
        v.push(p.parse::<i32>()?); // Propagate error using `?`
    }
    Ok(v)
}

pub fn vec2line<T: ToString>(output: Vec<T>, sep: &str)->String{
    output.iter()
        .map(|num| num.to_string())
        .collect::<Vec<_>>()
        .join(sep)
}

pub fn swap_vec_elements<T>(input: &mut Vec<T>, p1: usize, p2: usize){
    if p2 > p1 {
        let (left, right) = input.split_at_mut(p2);
        std::mem::swap(&mut left[p1], &mut right[0]);
    } else {
        let (left, right) = input.split_at_mut(p1);
        std::mem::swap(&mut left[p2], &mut right[0]);
    }
}

#[allow(dead_code)]
pub fn write_vec_to_file(vec: Vec<Vec<char>>, filename: &str) -> io::Result<()> {
    let mut file = File::create(filename)?;
    for line in vec {
        writeln!(file, "{}", line.iter().collect::<String>())?;
    }
    Ok(())
}