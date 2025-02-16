use std::cmp::Reverse;
use std::collections::{BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::rc::Rc;
use crate::geometry::{Canvas, Direction};
use crate::utils::{assert_display, Label, Solve};

pub(crate) struct Advent {
    label: Label,
    canvas: Canvas
}
impl Default for Advent {
    fn default() -> Self{
        Self{
            label: Label::new(24, 2016),
            canvas: Canvas::default()
        }
    }
}

impl Advent{
    fn compute_distances(&self, elements: BTreeSet<Rc<char>> )->Result<HashMap<(char, char), usize>,String>{
        let blocks = self.canvas.try_locate_element(&'#')?;

        let num_locations: HashSet<_> = self.canvas.elements()
            .iter()
            .filter(|(k, _)| !matches!(***k, '#' | '.'))
            .flat_map(|(_, v)| v.iter().cloned())
            .collect();

        let mut pair_distances: HashMap<(char, char), usize> = HashMap::new();
        for el in &elements {
            let from = self.canvas.try_locate_element(&el)?.into_iter().next().unwrap();
            let mut visited = HashSet::from([from.clone()]);
            let mut queue = VecDeque::from([(from.clone(), 0)]);

            while let Some((p, step)) = queue.pop_front() {
                for d in Direction::base() {
                    let p_next = &p + &d;
                    if !blocks.contains(&p_next) && visited.insert(p_next.clone()) {
                        if num_locations.contains(&p_next) {
                            let ch = *self.canvas.get_element(&*p_next).unwrap();
                            pair_distances.insert((**el, ch), step + 1);
                            pair_distances.insert((ch, **el), step + 1);
                        }
                        queue.push_back((p_next, step + 1));
                    }
                }
            }
        }
        Ok(pair_distances)
    }

    fn solve(&self,
             return_to_start: bool,
             result_prd: usize,
             part: u8) -> Result<String, String> {
        self.check_input(Some(part))?;

        let elements: BTreeSet<_> = self.canvas.get_element_set()
            .into_iter()
            .filter(|e| !matches!(**e, '.' | '#'))
            .collect();
        let pair_distances = self.compute_distances(elements.clone())?;

        let mut queue = BinaryHeap::from([Reverse((0, '0', elements.iter().filter(|e| ***e != '0').map(|e| **e).collect::<Vec<char>>()))]);
        let mut solution = None;

        while let Some(Reverse((dist, last, to_visit))) = queue.pop() {
            for &el in &to_visit {
                let next_dist = dist + pair_distances[&(last, el)];
                if to_visit.len() == 1 {
                    let final_dist = if return_to_start { next_dist + pair_distances[&('0', el)] } else { next_dist };
                    solution = Some(solution.map_or(final_dist, |s: usize| s.min(final_dist)));
                } else if solution.map_or(true, |s| s > next_dist) {
                    let next_to_visit: Vec<_> = to_visit.iter().filter(|&&x| x != el).copied().collect();
                    queue.push(Reverse((next_dist, el, next_to_visit)));
                }
            }
        }
        match solution{
            None => Err(String::from("Not solution found")),
            Some(d) => assert_display(d, None, result_prd, "Fewest number of steps", false)
        }
    }
}

impl Solve for Advent {

    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn get_canvas_mut(&mut self) -> Option<&mut Canvas>{
        Some(&mut self.canvas)
    }

    fn info(&self) -> Result<(), String>{
        self.check_input(None)?;
        println!("Canvas shape: {:?}", self.canvas.shape());
        let mut elements = self.canvas.get_element_set();
        elements.remove(&Rc::new('.'));
        elements.remove(&Rc::new('#'));
        println!("Locations {:?}", elements);
        Ok(())
    }
    fn compute_part1_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.solve(false, 502, 1)
    }
    fn compute_part2_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.solve(true, 724, 2)
    }
}

