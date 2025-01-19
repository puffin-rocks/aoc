use std::collections::HashMap;
use std::fs;
use regex::Regex;
use crate::utils::{assert_display, write_vec_to_file, Label, Solve};

const SCREEN_WIDTH: usize = 50;
const SCREEN_HEIGHT: usize = 6;

#[derive(Debug)]
enum Command{
    Rect(usize,usize),
    RotateRow(usize,usize),
    RotateCol(usize,usize)
}

impl Command{
    fn apply(&self, screen: &mut HashMap<(usize,usize), bool>){
        match self{
            Command::Rect(width, height) =>{
                for i in 0..*width{
                    for j in 0..*height{
                        screen.insert((j,i), true);
                    }
                }
            },
            Command::RotateRow(row_ix,shift) =>{
                let mut row = [false; SCREEN_WIDTH];
                for ((i,j), v) in screen.iter(){
                    if i == row_ix{
                        row[*j] = *v;
                    }
                }
                for j in 0..SCREEN_WIDTH{
                    screen.insert((*row_ix, (j+shift)%SCREEN_WIDTH), row[j]);
                }
            },
            Command::RotateCol(col_ix, shift) =>{
                let mut col = [false; SCREEN_HEIGHT];
                for ((i,j), v) in screen.iter(){
                    if j == col_ix{
                        col[*i] = *v;
                    }
                }
                for i in 0..SCREEN_HEIGHT{
                    screen.insert(((i+shift)%SCREEN_HEIGHT, *col_ix), col[i]);
                }
            }
        }
    }
}
pub(crate) struct Advent {
    label: Label,
    commands: Vec<Command>
}
impl Default for Advent {
    fn default() -> Self{
        Self{
            label: Label::new(8, 2016),
            commands: Vec::new()
        }
    }
}

impl Solve for Advent {

    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn add_record_from_line(&mut self, line: String) -> Result<(), std::num::ParseIntError>{
        let re = Regex::new(r"rect (\d+)x(\d+)").unwrap();
        if let Some(captures) = re.captures(line.as_str()) {
            let width = captures.get(1).unwrap().as_str().parse::<usize>()?;
            let height = captures.get(2).unwrap().as_str().parse::<usize>()?;
            self.commands.push(Command::Rect(width,height));
        }
        else {
            let re = Regex::new(r"rotate row y=(\d+) by (\d+)").unwrap();
            if let Some(captures) = re.captures(line.as_str()) {
                let row = captures.get(1).unwrap().as_str().parse::<usize>()?;
                let shift = captures.get(2).unwrap().as_str().parse::<usize>()?;
                self.commands.push(Command::RotateRow(row,shift));
            }else{
                let re = Regex::new(r"rotate column x=(\d+) by (\d+)").unwrap();
                if let Some(captures) = re.captures(line.as_str()) {
                    let col = captures.get(1).unwrap().as_str().parse::<usize>()?;
                    let shift = captures.get(2).unwrap().as_str().parse::<usize>()?;
                    self.commands.push(Command::RotateCol(col,shift));
                }else{
                    "invalid".parse::<i32>()?;
                }
            }
        }
        Ok(())
    }

    fn info(&self) -> Result<(), String>{
        self.check_input(None)?;
        println!("Number of commands: {}", self.commands.len());
        Ok(())
    }
    fn compute_part1_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.check_input(Some(1))?;
        let mut screen: HashMap<(usize,usize), bool> = HashMap::new();
        for i in 0..SCREEN_WIDTH {
            for j in 0..SCREEN_HEIGHT {
                screen.insert((j,i), false);
            }
        }
        for command in &self.commands{
            command.apply(&mut screen);
        }
        let result = screen.iter().filter(|(_, &v)| v).count();

        //create picture of display for part 2
        let prefix = format!("output/{}/", self.label.year);
        let _ = fs::create_dir_all(prefix.clone());
        let filename = format!("{}code.txt", prefix);

        let mut vec:Vec<Vec<char>> = Vec::new();
        for j in 0..SCREEN_HEIGHT {
            let mut row: Vec<char> = Vec::new();
            for i in 0..SCREEN_WIDTH {
                if *screen.get(&(j,i)).unwrap(){
                    row.push('\u{25A0}')
                }else{
                    row.push('.');
                }
            }
            vec.push(row);
        }
        let _ = write_vec_to_file(vec, &filename);

        assert_display(result, None, 110, "Number of lit pixels", false)
    }
    fn compute_part2_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.check_input(Some(2))?;
        Ok(String::from("No computation required"))
    }
}