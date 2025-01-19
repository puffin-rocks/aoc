use crate::utils::{assert_display, Label, Solve};
extern crate crypto;
use crypto::md5::Md5;
use crypto::digest::Digest;

pub(crate) struct Advent {
    label: Label,
    door_id: String
}
impl Default for Advent {
    fn default() -> Self{
        Self{
            label: Label::new(5, 2016),
            door_id: String::new()
        }
    }
}

impl Advent {
    fn solve(&self,
             hard: bool,
             result_prd: &str,
             part: u8
    ) -> Result<String, String> {
        self.check_input(Some(part))?;
        let mut hasher = Md5::new();
        let key = self.door_id.as_bytes();
        let mut password: [Option<char>; 8] = [None;8];
        let mut n_revealed = 0;
        for i in 0..std::u64::MAX {
            hasher.input(key);
            hasher.input(i.to_string().as_bytes());

            let mut output = [0; 16]; // An MD5 is 16 bytes
            hasher.result(&mut output);

            if output[0] | output[1] | (output[2] >> 4) == 0 {
                if hard {
                    let substr = output.iter()
                        .skip(2).take(2)
                        .map(|byte| format!("{:02x}", byte))
                        .collect::<String>();
                    if let Some(d) = substr.chars().nth(1).unwrap().to_digit(10) {
                        let d = d as usize;
                        if d < password.len() && password[d].is_none() {
                            password[d] = Some(substr.chars().nth(2).unwrap());
                            n_revealed += 1;
                        }
                    }
                }else{
                    let ch = output.iter()
                        .skip(2).take(1)
                        .map(|byte| format!("{:02x}", byte))
                        .collect::<String>().chars().nth(1).unwrap();
                    password[n_revealed] = Some(ch);
                    n_revealed += 1;
                }
            }
            hasher.reset();
            if password.len()==n_revealed{
                break;
            }
        }
        let password = password.iter().map(|&ch| ch.unwrap()).collect::<String>();
        assert_display(password, None, String::from(result_prd), "Password", false)
    }
}

impl Solve for Advent {

    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn add_record_from_line(&mut self, line: String) -> Result<(), std::num::ParseIntError>{
        self.door_id = line;
        Ok(())
    }

    fn info(&self) -> Result<(), String>{
        self.check_input(None)?;
        println!("Door ID: {}", self.door_id);
        Ok(())
    }
    fn compute_part1_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.solve(false, "1a3099aa", 1)
    }
    fn compute_part2_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.solve(true, "694190cd", 2)
    }
}