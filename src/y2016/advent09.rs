use regex::Regex;
use crate::utils::{assert_display, Label, Solve};

pub(crate) struct Advent {
    label: Label,
    compressed_file: String
}
impl Default for Advent {
    fn default() -> Self{
        Self{
            label: Label::new(9, 2016),
            compressed_file: String::new()
        }
    }
}

impl Solve for Advent {

    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn add_record_from_line(&mut self, line: String) -> Result<(), std::num::ParseIntError>{
        self.compressed_file = line;
        Ok(())
    }

    fn info(&self) -> Result<(), String>{
        self.check_input(None)?;
        println!("File length: {}", self.compressed_file.len());
        Ok(())
    }
    fn compute_part1_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.check_input(Some(1))?;
        let result = decompress_length(self.compressed_file.as_str(), true);
        assert_display(result, None, 150914, "Length of file", false)
    }
    fn compute_part2_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.check_input(Some(2))?;
        let result = decompress_length(self.compressed_file.as_str(), false);
        assert_display(result, None, 11052855125, "Length of file", false)
    }
}

fn decompress_length(compressed: &str, first_level: bool) -> usize {
    let mut loc = 0;
    let re = Regex::new(r"\((\d+)x(\d+)\)").unwrap();
    let mut cnt = 0;

    while loc < compressed.len() {
        if let Some(cap) = re.captures(&compressed[loc..]) {
            let (len, times): (usize, usize) = (cap[1].parse().unwrap(), cap[2].parse().unwrap());
            let m = cap.get(0).unwrap();
            let (start, end) = (m.start(), m.end());

            cnt += if first_level {
                compressed[loc..loc + start].len()
            } else {
                decompress_length(&compressed[loc..loc + start], first_level)
            };

            let next_loc = (loc + end + len).min(compressed.len());
            cnt += if first_level {
                len * times
            } else {
                decompress_length(&compressed[loc + end..next_loc], first_level) * times
            };

            loc = next_loc;
        } else {
            cnt += &compressed[loc..].len();
            break;
        }
    }

    cnt
}