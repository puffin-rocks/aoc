use std::collections::{BTreeSet, HashMap, HashSet};
use std::sync::Arc;
use rayon::prelude::*;
use crate::geometry::{CanvasAsync, Direction, Point2D};
use crate::utils::{Solve, Label, assert_display};

pub(crate) struct Advent {
    label: Label,
    canvas: CanvasAsync,
}


impl Default for Advent {
    fn default() -> Self {
        Self {
            label: Label::new(12, 2024),
            canvas: CanvasAsync::default(),
        }
    }
}
type APoint = Arc<Point2D>;
type AChar = Arc<char>;
type Edge = (Point2D, Point2D);

impl Advent{
    fn compute_buckets_and_walls(&self) -> (HashMap<&APoint, HashSet<Edge>>, HashMap<&AChar, Vec<HashSet<&APoint>>>) {
        let (vec_walls, vec_bucket_map): (Vec<HashMap<&APoint, HashSet<Edge>>>, Vec<HashMap<&AChar, Vec<HashSet<&APoint>>>>) =
            self.canvas.elements().par_iter()
                .map(|(k, v)| {
                    let mut bucket_map: HashMap<&AChar, Vec<HashSet<&APoint>>> = HashMap::new();
                    let mut walls: HashMap<&APoint, HashSet<Edge>> = HashMap::new();
                    let mut stack: BTreeSet<&APoint> = v.iter().collect();
                    let mut search_from: usize = 0;

                    loop {
                        let n = stack.len();
                        let mut new_stack: BTreeSet<&APoint> = BTreeSet::new();
                        for &p in stack.iter() {
                            let mut edges: HashSet<Edge> = HashSet::new();
                            let mut in_bucket = false;
                            if !bucket_map.contains_key(k) {
                                // start new bucket
                                let mut new_bucket: HashSet<&APoint> = HashSet::new();
                                new_bucket.insert(p);
                                bucket_map.entry(k).or_insert_with(Vec::new).push(new_bucket);
                                in_bucket = true;
                            }
                            // count walls
                            for d in Direction::base() {
                                let n_point = &**p + &d;
                                let edge = match d{
                                    Direction::Down => {
                                        (*p.clone(), &**p + &Direction::Right)
                                    },
                                    Direction::Up => {
                                        (&**p + &Direction::Up, &**p + &Direction::UpRight)
                                    },
                                    Direction::Left => {
                                        (*p.clone(), &**p + &Direction::Up)
                                    },
                                    Direction::Right =>{
                                        (&**p + &Direction::Right, &**p + &Direction::UpRight)
                                    },
                                    _ => unreachable!()
                                };
                                let neighbour = self.canvas.get_element(&n_point);
                                match neighbour {
                                    Some(&letter) => {
                                        if letter == **k {
                                            if !in_bucket {
                                                // try to add in the bucket of neighbour
                                                if let Some(bucket_list) = bucket_map.get_mut(&*k) {
                                                    for bucket in &mut bucket_list[search_from..] {
                                                        if bucket.contains(&Arc::new(n_point)) {
                                                            bucket.insert(p);
                                                            in_bucket = true;
                                                            break;
                                                        }
                                                    }
                                                }
                                            }
                                        } else {
                                            edges.insert(edge);
                                        }
                                    },
                                    None => {edges.insert(edge);},
                                };
                            }
                            walls.insert(p, edges);

                            if !in_bucket {
                                // start new bucket
                                new_stack.insert(p);
                            }
                        }

                        if new_stack.is_empty() {
                            break;
                        }
                        if n == new_stack.len() {
                            // no points were bucketed
                            let p = new_stack.pop_last().unwrap();
                            let mut new_bucket: HashSet<&APoint> = HashSet::new();
                            new_bucket.insert(p);
                            if let Some(bucket_list) = bucket_map.get(&*k) {
                                search_from = bucket_list.len();
                            }
                            bucket_map.entry(k).or_insert_with(Vec::new).push(new_bucket);
                        }
                        stack = new_stack;
                    }
                    (walls, bucket_map)
                })
                .unzip();  // This will give us two separate Vecs: one for walls, one for bucket_map

        // Merging the results
        let walls: HashMap<&APoint, HashSet<Edge>> = vec_walls.into_iter().fold(HashMap::new(), |mut acc, map| {
            acc.extend(map);
            acc
        });

        let bucket_map: HashMap<&AChar, Vec<HashSet<&APoint>>> = vec_bucket_map.into_iter().fold(HashMap::new(), |mut acc, map| {
            acc.extend(map);
            acc
        });
        (walls, bucket_map)
    }

}

impl Solve for Advent {
    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn get_canvas_async_mut(&mut self) -> Option<&mut CanvasAsync> {
        Some(&mut self.canvas)
    }

    fn info(&self) -> Result<(), String> {
        self.check_input(None)?;
        println!("Canvas shape: {:?}", self.canvas.shape());
        Ok(())
    }
    fn compute_part1_answer(&self, test_mode: bool) -> Result<String, String>{
        self.check_input(Some(1))?;
        //return Err(String::from("Skip"));
        let (walls, bucket_map) = self.compute_buckets_and_walls();
        // Merging result calculation
        let mut result = 0;
        for (_, bucket_vec) in bucket_map.iter() {
            for v in bucket_vec {
                let mut per = 0;
                for &p in v.iter() {
                    if let Some(edges) = walls.get(&p) {
                        per += edges.len();
                    }
                }
                result += v.len() * per;
            }
        }
        assert_display(result, Some(1930), 1486324, "Total price of fencing", test_mode)
    }
    fn compute_part2_answer(&self, test_mode: bool) -> Result<String, String>{
        self.check_input(Some(2))?;
        let (wall_map, bucket_map) = self.compute_buckets_and_walls();

        let mut result = 0;

        for bucket_vec in bucket_map.values() {
            for v in bucket_vec {
                let eobject: HashSet<_> = v.iter()
                    .flat_map(|p| wall_map.get(p).unwrap())
                    .cloned().collect();

                let mut edge_map: HashMap<Point2D, HashSet<Point2D>> = HashMap::new();
                for e in eobject {
                    edge_map.entry(e.0).or_default().insert(e.1);
                    edge_map.entry(e.1).or_default().insert(e.0);
                }

                let price: usize = edge_map
                    .iter()
                    .map(|(k, v)| {
                        if v.len() > 2 {
                            2
                        } else {
                            let pts: Vec<_> = v.iter().collect();
                            let (d1, d2) = ((pts[0] - k).to_point(), (pts[1] - k).to_point());
                            if d1.x() * d2.x() + d1.y() * d2.y() == 0 {
                                1
                            } else {
                                0
                            }
                        }
                    })
                    .sum();

                result += price * v.len();
            }
        }

        assert_display(result, Some(1206), 898684, "Total price of fencing", test_mode)
    }
}