use crate::utils::{assert_display, Label, Solve};

pub(crate) struct Advent {
    label: Label,
    n_elves: usize
}
impl Default for Advent {
    fn default() -> Self{
        Self{
            label: Label::new(19, 2016),
            n_elves: 0
        }
    }
}

impl Solve for Advent {

    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn add_record_from_line(&mut self, line: String) -> Result<(), std::num::ParseIntError>{
        self.n_elves = line.parse::<usize>()?;
        Ok(())
    }

    fn info(&self) -> Result<(), String>{
        self.check_input(None)?;
        println!("Number of elves: {}", self.n_elves);
        Ok(())
    }
    fn compute_part1_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.check_input(Some(1))?;
        let mut elves: Vec<(usize,usize)> = Vec::new();
        for i in 0..self.n_elves {
            elves.push((1, i+1));
        }
        let mut last_has_present = false;
        while elves.len()>1 {
            if last_has_present{
                elves.last_mut().unwrap().0+=elves[0].0;
                elves[0].0=0;
            }
            for i in 0..elves.len() - 1 {
                if elves[i].0 > 0 && elves[i + 1].0 > 0 {
                    elves[i].0 += elves[i + 1].0;
                    elves[i + 1].0 = 0;
                }
            }
            last_has_present = elves.last().unwrap().0 > 0;
            elves = elves.into_iter().filter(|&x | x.0>0).collect();
        }
        assert_display(elves.last().unwrap().1, None, 1830117, "Elf's number", false)
    }
    fn compute_part2_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.check_input(Some(2))?;

        let mut elves: Vec<usize> = (1..=self.n_elves).collect();
        let mut i = 0;
        let mut first_across: Option<usize> = None;
        let mut n_across = 0;

        while elves.len() > 1 {
            let shift = (elves.len() - n_across) / 2;
            let i_across = (i + shift + n_across) % elves.len();

            if first_across.is_none() {
                first_across = Some(i_across);
            }

            elves[i_across] = 0;
            n_across += 1;

            if let Some(v) = first_across {
                if v == i + 1 {
                    let x = elves[i];
                    elves = elves.into_iter().filter(|&x| x > 0).collect();
                    first_across = None;
                    n_across = 0;
                    i = elves.iter().position(|&v| v == x).unwrap_or(0) + 1;
                } else {
                    i += 1;
                }
            } else {
                i += 1;
            }

            if i >= elves.len() {
                elves = elves.into_iter().filter(|&x| x > 0).collect(); // Ensure removal of eliminated elves
                first_across = None;
                n_across = 0;
                i = 0;
            }
        }
        assert_display(*elves.last().unwrap(), None, 1417887, "Elf's number", false)
    }
}