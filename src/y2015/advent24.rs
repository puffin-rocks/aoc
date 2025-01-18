use std::collections::{BTreeMap, HashSet};
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
            while let Some(((n, pd, s), content)) = stack.pop_first() {
                if n+1<=min_size{
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
        Ok(())
    }
    fn compute_part1_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.solve(3usize, 11266889531, 1)
    }
    fn compute_part2_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.solve(4usize, 77387711, 2)
    }
}

fn do_split_exist(packages: &mut Vec<usize>, n_buckets: usize, target_weight: usize) -> bool {
    use std::collections::HashSet;

    // Sort in descending order to prioritize placing larger elements first.
    packages.sort_unstable_by(|a, b| b.cmp(a));

    let mut stack: Vec<(Vec<usize>, usize)> = vec![(vec![0; n_buckets], 0)]; // (bucket sums, index of current package)
    let mut seen = HashSet::new();

    while let Some((mut bucket_sums, idx)) = stack.pop() {
        // If we've already visited this state, skip it.
        if !seen.insert((bucket_sums.clone(), idx)) {
            continue;
        }

        // If all packages have been placed, we have a valid distribution.
        if idx == packages.len() {
            return true;
        }

        let package = packages[idx];
        for i in 0..n_buckets {
            // Skip if adding this package exceeds the target weight.
            if bucket_sums[i] + package > target_weight {
                continue;
            }

            bucket_sums[i] += package;
            stack.push((bucket_sums.clone(), idx + 1));
            bucket_sums[i] -= package;

            // If the bucket is empty, stop exploring other buckets to avoid redundant states.
            if bucket_sums[i] == 0 {
                break;
            }
        }
    }

    false
}