use std::collections::{HashMap};
use itertools::Itertools;
use regex::Regex;
use crate::utils::{assert_display, Label, Solve};

#[derive(Debug)]
struct Room {
    name: String,
    sector_id: usize,
    checksum: String,
}

impl Room {
    fn new(name: String, sector_id: usize, checksum: String) -> Self {
        Self {
            name,
            sector_id,
            checksum,
        }
    }

    fn is_real(&self) -> bool {
        let mut counter: HashMap<char, usize> = HashMap::new();
        for s in self.name.chars().filter(|&c| c != '-') {
            *counter.entry(s).or_insert(0) += 1;
        }

        let sorted_chars = counter
            .iter()
            .map(|(k, v)| (*v, *k))
            .sorted_by(|(v1, k1), (v2, k2)| v2.cmp(v1).then_with(|| k1.cmp(k2)))
            .map(|(_, k)| k)
            .collect::<Vec<_>>();

        self.checksum
            .chars()
            .enumerate()
            .all(|(i, c)| sorted_chars.get(i).copied() == Some(c))
    }

    fn decrypt(&self) -> String {
        let alphabet: Vec<char> = ('a'..='z').collect();
        let letter_to_num = alphabet.iter().enumerate().map(|(i, ch)| (*ch, i)).collect::<HashMap<char, usize>>();
        self.name.chars().map(|c| if c == '-' { ' ' } else { *alphabet.get((letter_to_num.get(&c).unwrap() + self.sector_id) % 26).unwrap() }).collect::<String>()
    }
}

pub(crate) struct Advent {
    label: Label,
    rooms: Vec<Room>,
}
impl Default for Advent {
    fn default() -> Self {
        Self {
            label: Label::new(4, 2016),
            rooms: Vec::new(),
        }
    }
}

impl Solve for Advent {
    fn get_label(&self) -> &Label { &self.label }
    fn get_label_mut(&mut self) -> &mut Label { &mut self.label }

    fn add_record_from_line(&mut self, line: String) -> Result<(), std::num::ParseIntError> {
        let re = Regex::new(r"^(.*)-(\d+)\[([a-z]+)]$").unwrap();
        if let Some(captures) = re.captures(line.as_str()) {
            let name = captures.get(1).unwrap().as_str().to_string();
            let sector_id = captures.get(2).unwrap().as_str().parse::<usize>()?;
            let checksum = captures.get(3).unwrap().as_str().to_string();
            self.rooms.push(Room::new(name, sector_id, checksum));
        } else {
            "invalid".parse::<i32>()?;
        }
        Ok(())
    }

    fn info(&self) -> Result<(), String> {
        self.check_input(None)?;
        println!("Number of rooms: {}", self.rooms.len());
        Ok(())
    }
    fn compute_part1_answer(&self, _test_mode: bool) -> Result<String, String> {
        self.check_input(Some(1))?;
        let result = self.rooms.iter().filter_map(|r| if r.is_real() { Some(r.sector_id) } else { None }).sum::<usize>();
        assert_display(result, None, 409147, "Sum of real room ids", false)
    }
    fn compute_part2_answer(&self, _test_mode: bool) -> Result<String, String> {
        self.check_input(Some(2))?;
        match self.rooms.iter()
            .filter_map(|r| if r.is_real() && r.decrypt().contains("north") { Some(r.sector_id) } else { None })
            .take(1).collect::<Vec<_>>().pop() {
            Some(result) => assert_display(result, None, 991, "North Pole objects room id", false),
            None => Err(String::from("No solution found"))
        }
    }
}