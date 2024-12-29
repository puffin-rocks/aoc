use crate::hashset;
use crate::utils::{assert_display, Label, Solve};

pub(crate) struct Advent {
    label: Label,
    lines: Vec<String>
}
impl Default for Advent {
    fn default() -> Self{
        Self{
            label: Label::new(5, 2015),
            lines: Vec::new()
        }
    }
}

impl Solve for Advent {

    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn add_record_from_line(&mut self, line: String) -> Result<(), std::num::ParseIntError>{
        self.lines.push(line);
        Ok(())
    }

    fn info(&self) -> Result<(), String>{
        self.check_input(None)?;
        println!("Number of lines: {}", self.lines.len());
        Ok(())
    }

    fn compute_part1_answer(&self, _: bool) -> Result<String, String>{
        self.check_input(Some(1))?;
        let forbidden = ["ab", "cd", "pq", "xy"];
        let vowels = hashset!['a', 'e', 'i', 'o', 'u'];
        let mut cnt = 0;
        for line in self.lines.iter(){
            let mut is_nice: Option<bool> = None;
            for s in forbidden{
                if line.contains(s){
                    is_nice = Some(false);
                    break;
                }
            }
            if is_nice.is_none(){
                let mut has_pair = false;
                let mut n_vowels =
                if vowels.contains( &line.chars().nth(0).unwrap_or('_')){
                    1
                }else{
                    0
                };
                for p in line.chars().collect::<Vec<_>>().windows(2){
                    if !has_pair{
                        has_pair = p[0]==p[1];
                    }
                    if vowels.contains(&p[1]){
                        n_vowels+=1;
                    }
                    if n_vowels>2 && has_pair{
                        is_nice = Some(true);
                        break;
                    }
                }
            }
            if let Some(is_nice) = is_nice{
                if is_nice{
                    cnt+=1;
                }
            }
        }
        assert_display(cnt, None, 238, "Number of nice strings", false);
    }
    //
    // fn compute_part2_answer(&self,  _: bool) -> Result<String, String>{
    //     self.check_input(Some(2))?;
    //     Err(String::from("Part 2 not implemented yet"))
    // }
}