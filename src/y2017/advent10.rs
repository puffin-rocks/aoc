use crate::utils::{assert_display, swap_vec_elements, Label, Solve};

pub(crate) struct Advent {
    label: Label,
    numbers1: Vec<usize>,
    numbers2: Vec<u8>
}
impl Default for Advent {
    fn default() -> Self{
        Self{
            label: Label::new(10, 2017),
            numbers1: Vec::new(),
            numbers2: Vec::new()
        }
    }
}

impl Solve for Advent {

    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn add_record_from_line(&mut self, line: String) -> Result<(), std::num::ParseIntError>{
        self.numbers2 =
        line.chars()
            .map(|ch| ch.encode_utf8(&mut [0;1]).as_bytes()[0])
            .collect::<Vec<u8>>();
        self.numbers2.extend([17, 31, 73, 47, 23]);
        self.numbers1 = line.split(",")
            .map(|x| x.parse::<usize>().expect("Cannot parse number"))
            .collect::<Vec<_>>();
        Ok(())
    }

    fn info(&self) -> Result<(), String>{
        self.check_input(None)?;
        println!("Number count: {}", self.numbers1.len());
        Ok(())
    }
    fn compute_part1_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.check_input(Some(1))?;
        let n = 256;
        let mut encryption_list = (0..n).collect::<Vec<usize>>();
        let mut location = 0;
        let mut skip = 0;
        for el in self.numbers1.iter(){
            let mut p = 0;
            let m = *el;
            while p<m/2{
                swap_vec_elements::<usize>(&mut encryption_list, (p+location)%n, (m+location-1-p)%n);
                p += 1;
            }
            location=(location+m+skip)%n;
            skip+=1;
        }
        let result = encryption_list[0]*encryption_list[1];
        assert_display(result, None, 40132, "Check", false)

    }
    fn compute_part2_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.check_input(Some(2))?;
        assert_display(knot_hash(&self.numbers2).as_str(), None, "35b028fe2c958793f7d5a61d07a008c8", "Knot Hash", false)
    }
}

pub fn knot_hash(input: &Vec<u8>)->String{
    let n = 256;
    let mut encryption_list = (0..n).collect::<Vec<usize>>();
    let mut location = 0;
    let mut skip = 0;
    while skip<64*input.len() {
        for el in input.iter() {
            let mut p = 0;
            let m = *el as usize;
            while p < m / 2 {
                swap_vec_elements::<usize>(&mut encryption_list,
                                           (p + location) % n,
                                           (m + location - 1 - p) % n);
                p += 1;
            }
            location = (location + m + skip) % n;
            skip+=1;
        }
    }
    const D: usize = 16;
    let mut dense_hash: [u8; D] = [0; D];
    for i in 0..D{
        let mut v = encryption_list[D*i];
        for j in 1..D{
            v^=encryption_list[D*i+j];
        }
        dense_hash[i] = v as u8;
    }
    dense_hash.iter()
        .map(|num|  format!("{:02x}", num))
        .collect::<Vec<_>>()
        .join("")
}