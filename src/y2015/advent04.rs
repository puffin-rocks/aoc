use crate::utils::{assert_display, Label, Solve};
use md5;

extern crate crypto;
use crypto::md5::Md5;
use crypto::digest::Digest;


pub(crate) struct Advent {
    label: Label,
    line: String
}
impl Default for Advent {
    fn default() -> Self{
        Self{
            label: Label::new(4, 2015),
            line: String::new()
        }
    }
}

impl Advent{
    fn _solve(&self, pattern: &str)->usize{
        //simple
        let mut i: usize = 1;
        loop{
            let result = format!("{:x}", md5::compute(format!("{}{}", self.line, i)));
            if result.starts_with(pattern){
                break;
            }else{
                i+=1;
            }
        }
        i
    }

    fn solve(&self, n_zeros: usize)->usize{
        //trying to speed up
        let mut hasher = Md5::new();

        let key = self.line.as_bytes();
        for i in 0..std::u64::MAX {
            hasher.input(key);
            hasher.input(i.to_string().as_bytes());

            let mut output = [0; 16]; // An MD5 is 16 bytes
            hasher.result(&mut output);

            let check = match n_zeros {
                5 => output[0] | output[1] | (output[2] >> 4),
                6 => output[0] | output[1] | output[2],
                _ => unreachable!()
            };
            if check ==0 {
                return i as usize;
            }
            hasher.reset();
        }
        0
    }
}

impl Solve for Advent {

    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn add_record_from_line(&mut self, line: String) -> Result<(), std::num::ParseIntError>{
        self.line= line;
        Ok(())
    }

    fn info(&self) -> Result<(), String>{
        self.check_input(None)?;
        Ok(())
    }

    fn compute_part1_answer(&self, _: bool) -> Result<String, String>{
        self.check_input(Some(1))?;
        let result = self.solve(5);
        assert_display(result, None, 282749, "Number for hash", false)
    }

    fn compute_part2_answer(&self,  _: bool) -> Result<String, String>{
        self.check_input(Some(2))?;
        let result = self.solve(6);
        assert_display(result, None, 9962624, "Number for hash", false)
    }
}