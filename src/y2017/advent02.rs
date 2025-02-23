use crate::utils::{assert_display, Label, Solve};

pub(crate) struct Advent {
    label: Label,
    rows: Vec<Vec<usize>>
}
impl Default for Advent {
    fn default() -> Self{
        Self{
            label: Label::new(2, 2017),
            rows: Vec::new()
        }
    }
}

impl Solve for Advent {

    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn add_record_from_line(&mut self, line: String) -> Result<(), std::num::ParseIntError>{
        self.rows.push(line.split_whitespace().map(|x| x.parse::<usize>().expect("Cannot convert to number")).collect::<Vec<_>>());
        Ok(())
    }

    fn info(&self) -> Result<(), String>{
        self.check_input(None)?;
        println!("Number of rows: {}", self.rows.len());
        Ok(())
    }
    fn compute_part1_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.check_input(Some(1))?;
        let checksum: usize = self.rows.iter().map(|x| x.iter().max().unwrap()-x.iter().min().unwrap()).sum();
        assert_display(checksum, None, 51139, "Checksum", false)
    }
    fn compute_part2_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.check_input(Some(2))?;
        let mut checksum = 0;
        for row in self.rows.iter(){
            let mut sorted_row = row.clone();
            sorted_row.sort_unstable();
            let div = find_pair(&sorted_row);
            match div{
                None => return Err(String::from("No solution found")),
                Some(v)=> checksum+=v
            };
        }
        assert_display(checksum, None, 272, "Checksum", false)
    }
}

fn find_pair(sorted_row: &Vec<usize>) -> Option<usize> {
    for i in 0..sorted_row.len()-1{
        for j in i+1..sorted_row.len(){
            if sorted_row[j]%sorted_row[i]==0{
                return Some(sorted_row[j]/sorted_row[i])
            }
        }
    }
    None
}