use crate::utils::{assert_display, Label, Solve};
const N_SIDES: usize = 3;
pub(crate) struct Advent {
    label: Label,
    triangles: Vec<[usize; N_SIDES]>
}
impl Default for Advent {
    fn default() -> Self{
        Self{
            label: Label::new(3, 2016),
            triangles: Vec::new()
        }
    }
}
impl Advent {
    fn solve(&self,
             triangles: &Vec<[usize;N_SIDES]>,
             result_prd: usize,
             part: u8
    ) -> Result<String, String> {
        self.check_input(Some(part))?;
        let n = triangles.iter().filter(|&t| {
            let m = t.iter().max().unwrap();
            let other_sides = t.iter().filter(|&s| s != m);
            if other_sides.count() ==2 {
                t.iter().filter(|&s| s != m).sum::<usize>() > *m
            }else{
                true
            }
        }
        ).count();
        assert_display(n, None, result_prd, "Number of valid triangles", false)
    }
}

impl Solve for Advent {

    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn add_record_from_line(&mut self, line: String) -> Result<(), std::num::ParseIntError>{
        let mut iter = line.split_whitespace();
        let mut triangle= [0usize;N_SIDES];
        for i in 0..N_SIDES {
            triangle[i] = iter.next().unwrap().parse::<usize>()?;
        }
        self.triangles.push(triangle);
        Ok(())
    }

    fn info(&self) -> Result<(), String>{
        self.check_input(None)?;
        println!("Number of triangles: {}", self.triangles.len());
        Ok(())
    }
    fn compute_part1_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.solve(&self.triangles,993, 1)
    }
    fn compute_part2_answer(&self, _test_mode: bool) -> Result<String, String>{
        let mut triangles: Vec<[usize;N_SIDES]> = vec![[0;N_SIDES];self.triangles.len()];
        for i in 0..self.triangles.len(){
            let r = i%N_SIDES;
            let d = i/N_SIDES;
            for j in 0..N_SIDES{
                triangles[d*N_SIDES+j][r] = self.triangles[i][j];
            }
        }
        self.solve(&triangles,1849, 2)
    }
}