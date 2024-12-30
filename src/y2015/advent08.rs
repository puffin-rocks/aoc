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
        for line in &self.lines {
            result += 2; // For the surrounding quotes
            let mut chars = line.chars();
            while let Some(ch) = chars.next() {
                if ch == '\\' {
                    if let Some(next) = chars.next() {
                        match next {
                            '\\' | '"' => {
                                result += 1;
                            }
                            'x' => {
                                if chars.next().is_some_and(|c1| {
                                    chars.next().is_some_and(|c2| {
                                        u8::from_str_radix(&format!("{c1}{c2}"), 16).is_ok()
                                    })
                                }) {
                                    result += 3;
                                }
                            }
                            _ => {}
                        }
                    }
                }
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