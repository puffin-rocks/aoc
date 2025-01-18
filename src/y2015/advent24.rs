use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
use crate::utils::{assert_display, Label, Solve};

pub(crate) struct Advent {
    label: Label,
    packages: Vec<usize>
}
impl Default for Advent {
    fn default() -> Self{
        Self{
            label: Label::new(24, 2015),
            packages: Vec::new()
        }
    }
}

impl Advent {
    fn solve(&self,
             n_buckets:usize,
             result_prd: usize,
             part: u8
    ) -> Result<String, String> {
        self.check_input(Some(part))?;
        let mut packages = self.packages.clone();
        packages.sort_unstable();
        let target_weight = packages.iter().sum::<usize>()/n_buckets;
        let mut min_size = packages.len();

        let mut stack: BTreeMap<(usize, usize, usize), Vec<usize>> = BTreeMap::new();
        let mut smallest_groups: BTreeMap<(usize, usize), Vec<usize>> = BTreeMap::new();
        stack.insert((0, 1, 0), Vec::new());
        for p in packages.iter().rev() {
            let mut stack_next: BTreeMap<(usize, usize, usize), Vec<usize>> = BTreeMap::new();
            stack.insert((0, 1, 0), Vec::new());
            while let Some(((n, pd, s), content)) = stack.pop_first() {
                if n+1<=min_size{//n+1<=min_size   { //
                    if s + p <= target_weight{ //
                        let mut content_next = content.clone();
                        content_next.push(*p);
                        if s + p == target_weight {
                            min_size=n+1;
                            smallest_groups.insert((n+1, pd*p), content_next);
                        } else {
                            stack_next.insert((n + 1, pd * p, s + p), content_next);
                            stack_next.insert((n, pd, s), content);
                        }
                    }else {
                        stack_next.insert((n, pd, s), content);
                    }
                }
            }
            stack = stack_next;
        }
        let mut min_quantum_entanglement = usize::MAX;
        for ((n, pd), content) in smallest_groups {
            if n>min_size{
                continue;
            }
            let mut rem_packages = packages.iter().cloned().collect::<HashSet<usize>>()
                .difference(&content.iter().cloned().collect::<HashSet<usize>>()).cloned().collect::<Vec<usize>>();
            if do_split_exist(&mut rem_packages, n_buckets-1, target_weight){
                min_quantum_entanglement = pd;
                break;
            }
        }
        assert_display(min_quantum_entanglement, None, result_prd, "Min quantum entanglement", false)
    }
}

impl Solve for Advent {

    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn add_record_from_line(&mut self, line: String) -> Result<(), std::num::ParseIntError>{
        self.packages.push(line.parse::<usize>()?);
        Ok(())
    }

    fn info(&self) -> Result<(), String>{
        self.check_input(None)?;
        println!("Number of packages: {}", self.packages.len());
        println!("Weight of packages: {}", self.packages.iter().sum::<usize>());
       // assert_eq!(self.packages.len(), self.packages.iter().collect::<HashSet<_>>().len());
        Ok(())
    }
    fn compute_part1_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.solve(3usize, 11266889531, 1)
    }
    fn compute_part2_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.solve(4usize, 77387711, 2)
        // let sum: usize = self.packages.iter().sum();
        // let target = sum / 4;
        // println!("{}", (1..self.packages.len()).find_map(|size| combinations(&self.packages.iter().map(|&x| x as u64 ).collect::<Vec<u64>>(), target as u64, size)).unwrap());
    }
}

fn do_split_exist(packages: &mut Vec<usize>, n_buckets:usize, target_weight: usize) -> bool{
    packages.sort_unstable();
    let mut stack: VecDeque<Vec<Vec<usize>>> = VecDeque::new();
    stack.push_back(vec![Vec::new(); n_buckets]);
    for p in packages.iter().rev() {
        let mut stack_next: VecDeque<Vec<Vec<usize>>> = VecDeque::new();
        while let Some(groups) = stack.pop_front() {
            for i in 0..groups.len() {
                if groups[i].iter().sum::<usize>() + p <= target_weight {
                    let mut groups_next = groups.clone();
                    groups_next[i].push(*p);
                    stack_next.push_back(groups_next);
                }
            }
        }
        stack = stack_next;
    }
    !stack.is_empty()
}


/// Check all combinations of `size` items returning `None` if no valid solution is found.
fn combinations(packages: &Vec<u64>, target: u64, size: usize) -> Option<u64> {
    // Mantain `size` indices, initially set to 0, 1, 2...
    let mut indices: Vec<_> = (0..size).collect();
    // Initial weight for first `size` items.
    let mut weight: u64 = packages.iter().take(size).sum();

    loop {
        // Check for success
        if weight == target {
            let product = indices.iter().map(|&i| packages[i]).product();
            println!("{:?}", indices.iter().map(|&i| packages[i]).collect::<Vec<_>>());
            return Some(product);
        }

        // Try to advance the last index. If the last index is at the end, then try to advance
        // the previous index until we reach the root.
        let mut depth = size - 1;
        while indices[depth] == packages.len() - size + depth {
            if depth == 0 {
                return None;
            }
            depth -= 1;
        }

        // Update the first index that is not at the end.
        let from = indices[depth];
        let to = indices[depth] + 1;
        indices[depth] = to;
        weight = weight - packages[from] + packages[to];
        depth += 1;

        // "Wrap" following indices to 1 more than the previous.
        while depth < size {
            let from = indices[depth];
            let to = indices[depth - 1] + 1;
            indices[depth] = to;
            weight = weight - packages[from] + packages[to];
            depth += 1;
        }
    }
}