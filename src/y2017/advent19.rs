use std::rc::Rc;
use crate::geometry::{Canvas, Direction, Point2D};
use crate::utils::{assert_display, vec2line, Label, Solve};

pub(crate) struct Advent {
    label: Label,
    canvas: Canvas,
}
impl Default for Advent {
    fn default() -> Self{
        Self{
            label: Label::new(19, 2017),
            canvas: Canvas::default(),
        }
    }
}

enum ResultType {
    Text(String),
    Number(usize),
}

impl Advent {
    fn solve(&self,
             result_prd: ResultType,
             part: u8
    ) -> Result<String, String> {
        self.check_input(Some(part))?;
        let mut curr_element ='|';
        let locs = self.canvas.try_locate_element(&curr_element)?;
        let mut start: Option<Rc<Point2D>> = None;
        for el in locs{
            if el.y() == &0{
                start = Some(el.clone());
                break
            }
        }
        if let Some(p) = start{
            let mut letters: Vec<char> = Vec::new();
            let mut curr_direction = Direction::Up;
            let mut curr_point = *p;
            let mut n_steps = 1;
            loop{
                let p_next = &curr_point + &curr_direction;
                if let Some(el) = self.canvas.get_element(&p_next){
                    match el{
                        '|' | '-' =>{ n_steps+=1; curr_point = p_next},
                        '+' => {
                            match curr_direction{
                                Direction::Up | Direction::Down => {
                                    for d in [Direction::Left, Direction::Right]{
                                        let p_test = &p_next + &d;
                                        if let Some(el_test) = self.canvas.get_element(&p_test){
                                            if *el_test != curr_element && *el_test!=' '{
                                                n_steps+=2;
                                                curr_direction = d;
                                                curr_point = p_test;
                                                curr_element = '-';
                                                break;
                                            }
                                        }
                                    }
                                },
                                Direction::Left | Direction::Right => {
                                    for d in [Direction::Up, Direction::Down]{
                                        let p_test = &p_next + &d;
                                        if let Some(el_test) = self.canvas.get_element(&p_test){
                                            if *el_test != curr_element && *el_test!=' '{
                                                n_steps+=2;
                                                curr_direction = d;
                                                curr_point = p_test;
                                                curr_element = '|';
                                                break;
                                            }
                                        }
                                    }
                                },
                                _ => unreachable!()
                            }
                        },
                        ' ' => {break}
                        _ => { n_steps+=1; letters.push(*el); curr_point = p_next;}
                    }
                }else{
                    break;
                }
            }
            match result_prd{
                ResultType::Text(text) => assert_display(vec2line(letters, ""), None, text, "Letters", false),
                ResultType::Number(num) => assert_display(n_steps, None, num, "Number of steps", false)
            }
        }else{
            Err(String::from("No start found"))
        }
    }
}

impl Solve for Advent {

    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn get_canvas_mut(&mut self) -> Option<&mut Canvas>{
        Some(&mut self.canvas)
    }

    fn info(&self) -> Result<(), String>{
        self.check_input(None)?;
        println!("Canvas shape: {:?}", self.canvas.shape());
        Ok(())
    }
    fn compute_part1_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.solve(ResultType::Text(String::from("VTWBPYAQFU")), 1)
    }
    fn compute_part2_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.solve(ResultType::Number(17358), 2)
    }
}