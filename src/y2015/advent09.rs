use crate::utils::{assert_display, Label, Solve};
use std::collections::{BTreeSet, HashMap, HashSet};

pub(crate) struct Advent {
    label: Label,
    edges: HashMap<(String, String), usize>,
    vertices: HashSet<String>,
}
impl Default for Advent {
    fn default() -> Self {
        Self {
            label: Label::new(9, 2015),
            edges: HashMap::new(),
            vertices: HashSet::new(),
        }
    }
}
impl Advent {
    fn solve(
        &self,
        shortest: bool,
        result_prd: usize,
        part: u8,
    ) -> Result<String, String> {
        self.check_input(Some(part))?;
        let header = if shortest{
            "Shortest distance"
        }else{
            "Longest distance"
        };
        let m: isize = if shortest{
            1
        }else{
            -1
        };
        let mut queue: BTreeSet<(isize, Vec<String>, Vec<String>)> = BTreeSet::new();
        for to in self.vertices.iter() {
            let mut s = self.vertices.clone();
            s.remove(to);
            queue.insert((0, vec![to.clone()], s.iter().cloned().collect()));
        }
        let distance: Option<usize>;
        loop {
            if let Some((d, visited, to_visit)) = queue.pop_first() {
                if to_visit.is_empty() {
                    distance = Some((m*d) as usize);
                    break;
                }
                for to in to_visit.iter() {
                    let mut s = to_visit.iter().cloned().collect::<HashSet<_>>();
                    s.remove(to);
                    let d = d + m * (*self
                        .edges
                        .get(&(visited.last().unwrap().clone(), to.clone()))
                        .unwrap() as isize);
                    let mut v = visited.clone();
                    v.push(to.clone());
                    queue.insert((d, v, s.iter().cloned().collect()));
                }
            } else {
                distance = None;
                break;
            }
        }
        match distance {
            Some(result) => assert_display(result, None, result_prd, header, false),
            None => Err(String::from("Solution not found")),
        }
    }
}

impl Solve for Advent {
    fn get_label(&self) -> &Label {
        &self.label
    }
    fn get_label_mut(&mut self) -> &mut Label {
        &mut self.label
    }

    fn add_record_from_line(&mut self, line: String) -> Result<(), std::num::ParseIntError> {
        if let Some((from_to, d)) = line.split_once(" = ") {
            let d = d.parse::<usize>()?;
            if let Some((from, to)) = from_to.split_once(" to ") {
                let (from, to) = (from.to_string(), to.to_string());
                self.vertices.insert(from.clone());
                self.vertices.insert(to.clone());
                self.edges.insert((from.clone(), to.clone()), d);
                self.edges.insert((to, from), d);
            }
        }
        Ok(())
    }

    fn info(&self) -> Result<(), String> {
        self.check_input(None)?;
        println!("Number of edges: {}", self.edges.len());
        Ok(())
    }

    fn compute_part1_answer(&self, _: bool) -> Result<String, String> {
        self.solve(true, 141, 1)
    }

    fn compute_part2_answer(&self, _: bool) -> Result<String, String> {
        self.solve(false, 736, 2)
    }
}
