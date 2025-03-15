use std::collections::{HashSet, VecDeque};
use crate::geometry::{Direction, Point2D};
use crate::utils::{assert_display, Label, Solve};
use crate::y2017::advent10::{knot_hash};
pub(crate) struct Advent {
    label: Label,
    key_string: String
}
impl Default for Advent {
    fn default() -> Self{
        Self{
            label: Label::new(14, 2017),
            key_string: String::new()
        }
    }
}

impl Solve for Advent {

    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn add_record_from_line(&mut self, line: String) -> Result<(), std::num::ParseIntError>{
        self.key_string = line;
        Ok(())
    }

    fn info(&self) -> Result<(), String>{
        self.check_input(None)?;
        Ok(())
    }
    fn compute_part1_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.check_input(Some(1))?;
        let grid = compute_grid(&self.key_string);
        let used = grid.iter().map(|hb| hb.chars().filter(|x| x == &'1').count()).sum();
        assert_display(used, None, 8230, "Number of used squares", false)
    }
    fn compute_part2_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.check_input(Some(2))?;
        let grid = compute_grid(&self.key_string);
        let mut used: HashSet<Point2D> = HashSet::new();
        let n = grid.len();
        for j in 0..n{
            for (i, ch) in grid[j].chars().enumerate(){
                if ch=='1'{
                    used.insert(Point2D::new(i,j));
                }
            }
        }
        let mut n_groups = 0usize;
        while let Some(seed) = used.iter().next(){
            let mut group: HashSet<Point2D> = HashSet::new();
            let mut stack: VecDeque<Point2D> = VecDeque::new();
            stack.push_back(seed.clone());
            group.insert(seed.clone());
            while let Some(p) = stack.pop_front(){
                for d in Direction::base(){
                    let p_next = &p+&d;
                    if p_next.is_out_of_bounds(n,n) | group.contains(&p_next) | !used.contains(&p_next){
                        continue
                    }
                    stack.push_back(p_next.clone());
                    group.insert(p_next);
                }
            }
            n_groups+=1;
            used = used.difference(&group).cloned().collect();
        }
        assert_display(n_groups, None, 1103, "Number of groups", false)
    }
}

fn encode(line: &String)->Vec<u8> {
    let mut tmp = line.chars()
        .map(|ch| ch.encode_utf8(&mut [0; 1]).as_bytes()[0])
        .collect::<Vec<u8>>();
    tmp.extend([17, 31, 73, 47, 23]);
    tmp
}

fn compute_grid(key_string: &String)->Vec<String>{
    (0..=127u8).into_iter().map(|i| {
        let line = format!("{}-{}", key_string, i);
        let h = knot_hash(&encode(&line));
        h.chars()
            .map(|ch| format!("{:04b}", ch.to_digit(16).unwrap()))
            .collect::<String>()
    }).collect::<Vec<_>>()
}