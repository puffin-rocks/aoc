use std::collections::HashSet;
use crate::utils::{assert_display, Label, Solve};

pub(crate) struct Advent {
    label: Label,
    ips: Vec<String>
}
impl Default for Advent {
    fn default() -> Self{
        Self{
            label: Label::new(7, 2016),
            ips: Vec::new()
        }
    }
}

impl Solve for Advent {

    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn add_record_from_line(&mut self, line: String) -> Result<(), std::num::ParseIntError>{
        self.ips.push(line);
        Ok(())
    }

    fn info(&self) -> Result<(), String>{
        self.check_input(None)?;
        println!("Number of ips: {}", self.ips.len());
        Ok(())
    }
    fn compute_part1_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.check_input(Some(1))?;
        let mut cnt = 0;
        for ip in &self.ips {
            let mut in_hypernet = false;
            let mut valid = false;

            for w in ip.chars().collect::<Vec<_>>().windows(4) {
                match w[0] {
                    '[' => in_hypernet = true,
                    ']' => in_hypernet = false,
                    _ => {}
                }

                if w.contains(&'[') || w.contains(&']') {
                    continue;
                }

                if w[0] != w[1] && w[0] == w[3] && w[1] == w[2] {
                    if in_hypernet {
                        valid = false;
                        break;
                    } else {
                        valid = true;
                    }
                }
            }

            if valid {
                cnt += 1;
            }
        }
        assert_display(cnt, None, 118, "Number of IPs supporting TLS", false)
    }
    fn compute_part2_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.check_input(Some(2))?;
        let mut cnt = 0;
        for ip in &self.ips {
            let mut hypernet = false;
            let mut patterns = HashSet::new();

            for w in ip.chars().collect::<Vec<_>>().windows(3) {
                match w[0] {
                    '[' => hypernet = true,
                    ']' => hypernet = false,
                    _ if !hypernet && w[0] != w[1] && w[0] == w[2] => {
                        patterns.insert([w[1], w[0], w[1]]);
                    }
                    _ => {}
                }
            }

            if patterns.is_empty() {
                continue;
            }

            let mut has_bab = false;
            for w in ip.chars().collect::<Vec<_>>().windows(3) {
                match w[0] {
                    '[' => hypernet = true,
                    ']' => hypernet = false,
                    _ if hypernet && patterns.contains(w) => {
                        has_bab = true;
                        break;
                    }
                    _ => {}
                }
            }

            if has_bab {
                cnt += 1;
            }
        }
        assert_display(cnt, None, 260, "Number of IPs supporting SSL", false)
    }
}