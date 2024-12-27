use crate::utils::{assert_display, Label, Solve};

pub(crate) struct Advent {
    label: Label,
    boxes: Vec<[usize;3]>
}
impl Default for Advent {
    fn default() -> Self{
        Self{
            label: Label::new(2, 2015),
            boxes: Vec::new()
        }
    }
}

impl Solve for Advent {

    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn add_record_from_line(&mut self, line: String) -> Result<(), std::num::ParseIntError>{
        let x = line.split("x")
            .map(|p|{p.parse::<usize>().expect("Cannot parse number")})
            .collect::<Vec<usize>>();
        self.boxes.push([x[0], x[1], x[2]]);
        Ok(())
    }

    fn info(&self) -> Result<(), String>{
        self.check_input(None)?;
        println!("Number of boxes: {}", self.boxes.len());
        Ok(())
    }

    fn compute_part1_answer(&self, _: bool) -> Result<String, String>{
        self.check_input(Some(1))?;
        let result:usize = self.boxes.iter().map(|b|{
            let areas = b.iter()
                .chain(b.iter().take(1)).collect::<Vec<_>>()
                .windows(2)
                .map(|d|d[0]*d[1]).collect::<Vec<_>>();
            areas.iter().sum::<usize>()*2+areas.iter().min().unwrap()
        }).sum();
        assert_display(result, None, 1586300, "Paper (sq. feet)", false)
    }

    fn compute_part2_answer(&self,  _: bool) -> Result<String, String>{
        self.check_input(Some(2))?;
        let result:usize = self.boxes.iter().map(|b|{
            let areas = b.iter()
                .chain(b.iter().take(1)).collect::<Vec<_>>()
                .windows(2)
                .map(|d|d[0]+d[1]).collect::<Vec<_>>();
            b.iter().product::<usize>() + 2*areas.iter().min().unwrap()
        }).sum();
        assert_display(result, None, 3737498, "Ribbon (feet)", false)
    }
}