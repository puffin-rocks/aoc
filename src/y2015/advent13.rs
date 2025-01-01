use std::collections::{BTreeSet, HashMap, HashSet};
use crate::utils::{assert_display, Label, Solve};

pub(crate) struct Advent {
    label: Label,
    edges: HashMap<(String,String), isize>,
    vertices: BTreeSet<String>
}
impl Default for Advent {
    fn default() -> Self{
        Self{
            label: Label::new(13, 2015),
            edges: HashMap::new(),
            vertices: BTreeSet::new()
        }
    }
}

impl Advent{
    fn solve(&self,
             include_host: bool,
             result_prd: isize,
             part: u8) -> Result<String, String> {
        self.check_input(Some(part))?;
        let mut edges = self.edges.clone();
        let host = String::from("Host");
        if include_host{
            for v in self.vertices.iter(){
                edges.insert((v.clone(), host.clone()), 0);
                edges.insert((host.clone(), v.clone()), 0);
            }
        }
        let mut queue: BTreeSet<(isize, Vec<String>, Vec<String>)> = BTreeSet::new();
        let mut s = self.vertices.clone();
        if include_host{
            s.insert(host);
        }
        let to = s.pop_first().expect("Vertex set is empty");
        queue.insert((0, vec![to.clone()], s.iter().cloned().collect()));

        let mut happiness: Option<isize> = None;
        loop {
            if let Some((h, seated, to_seat)) = queue.pop_first() {
                if to_seat.is_empty() {
                    let last = seated.last()
                        .expect("Seated list is empty")
                        .clone();
                    let first = seated.first()
                        .expect("Seated list is empty")
                        .clone();
                    let gain = edges
                        .get(&(last.clone(), first.clone()))
                        .unwrap()
                        + edges
                        .get(&(first.clone(), last.clone()))
                        .unwrap();
                    let h = h + gain;
                    match happiness {
                        None => happiness = Some(h),
                        Some(current) => {
                            if h > current {
                                happiness = Some(h);
                            }
                        }
                    }
                }
                for to in to_seat.iter() {
                    let mut s = to_seat.iter().cloned().collect::<HashSet<_>>();
                    s.remove(to);
                    let last = seated.last()
                        .expect("Seated list is empty")
                        .clone();
                    let gain = edges
                        .get(&(last.clone(), to.clone()))
                        .unwrap()
                        + edges
                        .get(&(to.clone(), last.clone()))
                        .unwrap();
                    let mut v = seated.clone();
                    v.push(to.clone());
                    queue.insert((h+gain, v, s.iter().cloned().collect()));
                }
            } else {
                happiness = None;
                break;
            }
            if queue.is_empty(){
                break;
            }
        }
        match happiness {
            Some(result) => assert_display(result, None, result_prd, "Happiness", false),
            None => Err(String::from("Solution not found")),
        }
    }
}

impl Solve for Advent {

    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn add_record_from_line(&mut self, line: String) -> Result<(), std::num::ParseIntError>{
        let tmp = line.split(" ").collect::<Vec<_>>();
        let p1 = tmp[0].to_string();
        let p2  =tmp.last().expect("Empty list").trim_end_matches('.').to_string();
        let mut value_abs = tmp[3].parse::<isize>()?;
        if tmp[2] == "lose"{
            value_abs=-1*value_abs;
        }
        self.edges.insert((p1.clone(), p2.clone()), value_abs);
        self.vertices.insert(p1);
        self.vertices.insert(p2);
        Ok(())
    }

    fn info(&self) -> Result<(), String>{
        self.check_input(None)?;
        println!("Number of guests: {}", self.vertices.len());
        println!("Number of preferences: {}", self.edges.len());
        Ok(())
    }

    fn compute_part1_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.solve(false, 618, 1)
    }

    fn compute_part2_answer(&self,  _: bool) -> Result<String, String>{
        self.solve(true, 618, 2)
    }
}