use std::rc::Rc;
use crate::geometry::{Canvas, Direction};
use crate::hashset;
use crate::utils::{assert_display, Label, Solve};

pub(crate) struct Advent {
    label: Label,
    instructions: Vec<String>
}
impl Default for Advent {
    fn default() -> Self{
        Self{
            label: Label::new(2, 2016),
            instructions: Vec::new()
        }
    }
}

impl Advent {
    fn solve(&self,
             keypad: Canvas,
             result_prd: String,
             part: u8
    ) -> Result<String, String> {
        self.check_input(Some(part))?;
        let forbidden = hashset![Some(&'.'), None];
        let mut position = keypad.try_locate_element(&'5')?.clone().pop_first().unwrap();
        let mut code: String = String::new();
        for i in self.instructions.iter(){
            for mve in i.chars(){
                let d = match mve{
                    'U' => Direction::Down,
                    'D' => Direction::Up,
                    'R' => Direction::Right,
                    'L' => Direction::Left,
                    _ => unreachable!()
                };
                let next = &*position + &d;
                if !forbidden.contains(&keypad.get_element(&next)){
                    position = Rc::new(next);
                }
            }
            code.push(*keypad.get_element(&*position).unwrap());
        }
        assert_display(code, None, result_prd, "Code", false)
    }
}

impl Solve for Advent {

    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn add_record_from_line(&mut self, line: String) -> Result<(), std::num::ParseIntError>{
        self.instructions.push(line);
        Ok(())
    }

    fn info(&self) -> Result<(), String>{
        self.check_input(None)?;
        println!("Number of steps per code digit: {:?}", self.instructions.iter().map(|x|x.len()).collect::<Vec<_>>());
        Ok(())
    }
    fn compute_part1_answer(&self, _test_mode: bool) -> Result<String, String>{
        let mut keypad: Canvas = Canvas::default();
        keypad.add_row(vec!['1','2','3']);
        keypad.add_row(vec!['4','5','6']);
        keypad.add_row(vec!['7','8','9']);
        self.solve(keypad, String::from("65556"), 1)
    }
    fn compute_part2_answer(&self, _test_mode: bool) -> Result<String, String>{
        let mut keypad: Canvas = Canvas::default();
        keypad.add_row(vec!['.','.','1','.','.']);
        keypad.add_row(vec!['.','2','3','4','.']);
        keypad.add_row(vec!['5','6','7','8','9']);
        keypad.add_row(vec!['.','A','B','C','.']);
        keypad.add_row(vec!['.','.','D','.','.']);
        self.solve(keypad, String::from("CB779"), 2)
    }
}