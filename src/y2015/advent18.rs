use std::rc::Rc;
use crate::geometry::{Canvas, Direction, Point2D};
use crate::hashset;
use crate::utils::{assert_display, Label, Solve};

pub(crate) struct Advent {
    label: Label,
    canvas: Canvas
}
impl Default for Advent {
    fn default() -> Self{
        Self{
            label: Label::new(18, 2015),
            canvas: Canvas::default()
        }
    }
}

impl Advent {
    fn solve(&self,
             stuck_corners: bool,
             result_prd: usize,
             part: u8
    ) -> Result<String, String> {
        self.check_input(Some(part))?;
        let (&width, &height) = self.canvas.shape();

        let (corners, mut canvas) = if stuck_corners {
            let corners = hashset!(Point2D::new(0,0),
                Point2D::new(0, height-1),
                Point2D::new(width-1, 0),
                Point2D::new(width-1, height-1)
            );
            let mut canvas = Canvas::default();
            for y in 0..height {
                let row = (0..width)
                    .map(|x| {
                        let p = Point2D::new(x, y);
                        let el = *self.canvas.get_element(&p).unwrap();
                        if corners.contains(&p) {
                            '#'
                        } else {
                            el
                        }
                    }
                    ).collect();
                canvas.add_row(row);
            }
            (corners, canvas)
        }else{
            (hashset!(), self.canvas.clone())
        };

        for _ in 0..100 {
            let mut next_canvas = Canvas::default();

            for y in 0..height {
                let row = (0..width)
                    .map(|x| {
                        let p = Point2D::new(x, y);
                        let el = canvas.get_element(&p).unwrap();
                        if corners.contains(&p){
                            '#'
                        }
                        else {
                            let count_on = [&Direction::base()[..], &Direction::diagonal()[..]].concat()
                                .iter()
                                .filter_map(|d| canvas.get_element(&(&p + d)))
                                .filter(|&&el| el == '#' || corners.contains(&p) )
                                .count();

                            match el {
                                '#' if (2..=3).contains(&count_on) => '#',
                                '.' if count_on == 3 => '#',
                                _ => '.',
                            }
                        }
                    })
                    .collect();

                next_canvas.add_row(row);
            }

            canvas = next_canvas;
        }
        let result = match canvas.elements().get(&Rc::new('#')){
            None => 0,
            Some(points) => points.len()
        };
        assert_display(result, None, result_prd, "Number of lights", false)
    }
}

impl Solve for Advent {

    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn get_canvas_mut(&mut self) -> Option<&mut Canvas> {
        Some(&mut self.canvas)
    }
    fn info(&self) -> Result<(), String>{
        self.check_input(None)?;
        println!("Canvas shape: {:?}", self.canvas.shape());
        Ok(())
    }
    fn compute_part1_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.solve(false, 821, 1)
    }
    fn compute_part2_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.solve(true, 886, 2)
    }
}