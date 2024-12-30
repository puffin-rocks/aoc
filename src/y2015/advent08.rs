use crate::utils::{assert_display, Label, Solve};

pub(crate) struct Advent {
    label: Label,
    lines: Vec<String>
}
impl Default for Advent {
    fn default() -> Self{
        Self{
            label: Label::new(8, 2015),
            lines: Vec::new()
        }
    }
}

impl Solve for Advent {

    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn add_record_from_line(&mut self, line: String) -> Result<(), std::num::ParseIntError>{
       // println!("{}", line);
        // println!("{}", u8::from_str_radix("ff", 16)?);
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
        let mut result = 0;
        for line in self.lines.iter(){
            result +=2;
            let mut i = 0;
            while i<line.len(){
                let ch = line.chars().nth(i).unwrap();
                if ch=='\\'{
                    if let Some(ch) = line.chars().nth(i+1){
                        match ch{
                            '\\' | '\"' =>{
                                result +=1;
                                i+=1;
                            }
                            'x' =>{
                                if let (Some(ch1), Some(ch2)) = (line.chars().nth(i+2),line.chars().nth(i+3)){
                                    let mut s = String::new();
                                    s.push(ch1);
                                    s.push(ch2);
                                    if u8::from_str_radix(s.as_str(), 16).is_ok(){
                                        result +=3;
                                        i+=3;
                                    }
                                }
                            }
                            _ =>{}
                        }
                    }
                }
                i+=1;
            }
        }
        assert_display(result, None, 1371, "String overhead", false)
    }

    fn compute_part2_answer(&self,  _: bool) -> Result<String, String>{
        self.check_input(Some(2))?;
        let mut result = 0;
        for line in self.lines.iter() {
            result += 2;
            for ch in line.chars() {
                if ch == '\\' || ch == '\"' {
                    result += 1;
                }
            }
        }
        assert_display(result, None, 2117, "String overhead", false)
    }
}