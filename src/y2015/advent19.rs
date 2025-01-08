use std::collections::{HashMap, HashSet};
use regex::Regex;
use crate::utils::{assert_display, Label, Solve};

pub(crate) struct Advent {
    label: Label,
    read_replacement: bool,
    replacements: HashMap<String, HashSet<String>>,
    molecule: String
}
impl Default for Advent {
    fn default() -> Self{
        Self{
            label: Label::new(19, 2015),
            read_replacement: true,
            replacements: HashMap::new(),
            molecule: String::new()
        }
    }
}

fn all_replacements(molecule: &String, replacements: &HashMap<String,HashSet<String>>) -> HashSet<String>{
    let mut molecules: HashSet<String> = HashSet::new();
    for i in 0..molecule.len() {
        for (k, elements) in replacements {
            if let Some(suffix) = molecule[i..].strip_prefix(k) {
                for el in elements {
                    let new_molecule = [&molecule[..i], el, suffix].concat();
                    molecules.insert(new_molecule);
                }
            }
        }
    }
    molecules
}

use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Eq, PartialEq)]
struct ByLength(String, usize);

impl Ord for ByLength {
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.len().cmp(&self.0.len()).then_with(|| other.1.cmp(&self.1))
    }
}

impl PartialOrd for ByLength {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Solve for Advent {

    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn add_record_from_line(&mut self, line: String) -> Result<(), std::num::ParseIntError>{
        if line.is_empty(){
            self.read_replacement = false;
        }else{
            if self.read_replacement{
                let re = Regex::new(r"(\w+) => (\w+)").unwrap();
                if let Some(captures) = re.captures(line.as_str()) {
                    let key =  String::from(captures.get(1).unwrap().as_str());
                    let value =  String::from(captures.get(2).unwrap().as_str());
                    self.replacements.entry(key).or_insert_with(HashSet::new).insert(value);
                }
            }else{
                self.molecule = line;
            }
        }
        Ok(())
    }

    fn info(&self) -> Result<(), String>{
        self.check_input(None)?;
        println!("Number of replacements: {}", self.replacements.len());
        println!("Molecule length: {}", self.molecule.len());
        Ok(())
    }
    fn compute_part1_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.check_input(Some(1))?;
        let molecules = all_replacements(&self.molecule, &self.replacements);
        let result = molecules.len();
        assert_display(result, None, 518, "Number of molecules", false)
    }
    fn compute_part2_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.check_input(Some(2))?;

        let mut inv_replacements: HashMap<String, HashSet<String>> = HashMap::new();
        for (k, elements) in self.replacements.iter(){
            for el in elements.iter(){
                inv_replacements.entry(el.clone()).or_insert_with(HashSet::new).insert(k.clone());
            }
        }

        let mut heap = BinaryHeap::new();
        let mut step = 0;
        heap.push(ByLength(self.molecule.clone(), step));

        while let Some(ByLength(molecule, curr_step)) = heap.pop() {
            if molecule=="e"{
                step = curr_step;
                break;
            }
            for m in all_replacements(&molecule, &inv_replacements){
                heap.push(ByLength(m, curr_step+1));
            }
        }
        assert_display(step, None, 200, "Min number of steps", false)
    }
}

