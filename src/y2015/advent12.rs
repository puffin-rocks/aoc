use crate::utils::{assert_display, Label, Solve};
use serde_json::Value;

pub(crate) struct Advent {
    label: Label,
    json: String,
}
impl Default for Advent {
    fn default() -> Self{
        Self{
            label: Label::new(12, 2015),
            json: String::new()
        }
    }
}

impl Solve for Advent {

    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn add_record_from_line(&mut self, line: String) -> Result<(), std::num::ParseIntError>{
        self.json = line;
        Ok(())
    }

    fn info(&self) -> Result<(), String>{
        self.check_input(None)?;
        println!("JSON length: {}", self.json.len());
        Ok(())
    }

    fn compute_part1_answer(&self, _: bool) -> Result<String, String>{
        self.check_input(Some(1))?;
        let parts = self.json.split(&[':', ',','[',']','{','}'][..]).collect::<Vec<_>>();
        let mut sum = 0;
        for p in parts{
            match p.parse::<isize>(){
                Ok(num) => sum+=num,
                Err(_) => {}
            }
        }
        assert_display(sum, None, 119433, "Sum of numbers", false)
    }

    fn compute_part2_answer(&self,  _: bool) -> Result<String, String>{
        self.check_input(Some(2))?;
        let parsed: Value = serde_json::from_str(self.json.as_str()).expect("Invalid JSON");
        let mut sum = 0;
        traverse(&parsed, &mut sum);
        assert_display(sum, None, 68466, "Sum of numbers", false)
    }
}

fn traverse(parsed: &Value, sum: &mut i64) {
    match parsed {
        Value::Object(map) => {
            if !map.values().any(|v| v.as_str() == Some("red")) {
                for value in map.values() {
                    traverse(value, sum);
                }
            }
        }
        Value::Array(arr) => {
            for value in arr {
                traverse(value, sum);
            }
        }
        Value::Number(num) if num.is_i64() => {
            if let Some(val) = num.as_i64() {
                *sum += val;
            }
        }
        _ => {}
    }
}