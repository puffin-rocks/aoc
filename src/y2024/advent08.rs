use std::collections::{BTreeSet};
use std::rc::Rc;
use crate::geometry::{Canvas, Point2D};
use crate::utils::{Solve, Label, assert_display};

pub(crate) struct Advent {
    label: Label,
    canvas: Canvas,
}

impl Default for Advent {
    fn default() -> Self {
        Self {
            label: Label::new(8, 2024),
            canvas: Canvas::default(),
        }
    }
}

impl Advent {
    fn solve(&self,
             one_step: bool,
             result_test: usize,
             result_prd: usize,
             test_mode: bool,
             part: u8) -> Result<String, String>{
        self.check_input(Some(part))?;
        let (&w, &h) = self.canvas.shape();
        let mut antinodes: BTreeSet<Rc<Point2D>> = BTreeSet::new();

        for (el, antennas_set) in self.canvas.elements() {
            if **el == '.' {
                continue;
            }
            let n = antennas_set.len();
            if n < 2 {
                continue;
            }

            let antennas: Vec<Rc<Point2D>> = antennas_set.iter().map(|rc| Rc::clone(rc)).collect();

            for i in 0..n - 1 {
                for j in i + 1..n {
                    let d = &*antennas[i] - &*antennas[j];

                    for start in [&antennas[j], &antennas[i]] {
                        let direction = if *start == antennas[j] { 1 } else { -1 };
                        let mut s = 2;

                        loop {
                            let p = &*Rc::clone(start) + &(&d * (s * direction));
                            if p.is_out_of_bounds(w, h) {
                                break;
                            }
                            antinodes.insert(Rc::new(p));
                            s += 1;
                            if one_step{
                                break
                            }
                        }
                    }
                }
            }
            if !one_step {
                antinodes.extend(antennas);
            }
        }
        assert_display(antinodes.len(), Some(result_test), result_prd, "Number of antinodes", test_mode)
    }
}

impl Solve for Advent {
    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}
    fn get_canvas_mut(&mut self) -> Option<&mut Canvas>{
        Some(&mut self.canvas)
    }

    fn info(&self) -> Result<(), String> {
        self.check_input(None)?;
        println!("Canvas shape: {:?}", self.canvas.shape());
        let mut elements = self.canvas.get_element_set();
        elements.remove(&Rc::new('.'));
        println!("Number of antenna types: {}", elements.len() );
        Ok(())
    }
    fn compute_part1_answer(&self, test_mode: bool) -> Result<String, String>{
        self.solve(true, 14, 357, test_mode, 1)
    }
    fn compute_part2_answer(&self, test_mode: bool) -> Result<String, String>{
        self.solve(false, 34, 1266, test_mode, 2)
    }
}