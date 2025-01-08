use std::collections::HashSet;
use crate::utils::{assert_display, Label, Solve};

pub(crate) struct Advent {
    label: Label,
    number: usize,
    use_bruteforce: bool
}
impl Default for Advent {
    fn default() -> Self{
        Self{
            label: Label::new(20, 2015),
            number: 0,
            use_bruteforce: false
        }
    }
}

impl Solve for Advent {

    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn apply_bruteforce(&mut self){
        println!("...Applying bruteforce...");
        self.use_bruteforce = true;
    }

    fn add_record_from_line(&mut self, line: String) -> Result<(), std::num::ParseIntError>{
        self.number = line.parse::<usize>()?;
        Ok(())
    }

    fn info(&self) -> Result<(), String>{
        self.check_input(None)?;
        println!("{}", self.number);
        Ok(())
    }
    fn compute_part1_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.check_input(Some(1))?;

        let mut n_houses = 1;
        while max_presents(n_houses)<self.number{
            n_houses*=10;
        }
        let min_number: usize;
        if self.use_bruteforce {
            let mut i = n_houses / 10;

            loop {
                let res = get_house_presents(i);
                if res >= self.number {
                    min_number = i;
                    break;
                }
                i+=1;
            }
        }
        else {
            let mut n_houses_upper = n_houses;
            let mut n_houses_lower = n_houses/10;
            let mut n_houses_med = (n_houses_upper+n_houses_lower)/2;
            loop {
                let result = max_presents(n_houses_med);

                if result > self.number {
                    n_houses_upper = n_houses_med;
                    n_houses_med = (n_houses_upper + n_houses_lower) / 2;
                } else if result < self.number {
                    n_houses_lower = n_houses_med;
                    n_houses_med = (n_houses_upper + n_houses_lower) / 2;
                } else {
                    min_number = n_houses_med;
                    break;
                }
                if n_houses_upper - n_houses_lower==1{
                    min_number = n_houses_upper;
                    break;
                }
            }
        }
        assert_display(min_number, None, 776160, "First house number", false)
    }
    fn compute_part2_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.check_input(Some(2))?;
        Err(String::from("Not implemented"))
    }
}

fn get_house_presents(n: usize) -> usize {
    let mut divisors = HashSet::new();
    let sqrt_n = (n as f64).sqrt() as usize;

    for i in 1..=sqrt_n {
        if n % i == 0 {
            divisors.insert(i); // Add the divisor
            if i != n / i {
                divisors.insert(n / i); // Add the complement divisor
            }
        }
    }
    divisors.iter().sum::<usize>()*10
}

fn max_presents(n_houses:usize)->usize{
    let mut presents = vec![0; n_houses+1];
    let mut n = 1;
    while n<=n_houses{
        let mut elf = n;
        while elf<presents.len(){
            presents[elf]+=n*10;
            elf+=n;
        }
        n+=1;
    }
    *presents.iter().max().unwrap()
}