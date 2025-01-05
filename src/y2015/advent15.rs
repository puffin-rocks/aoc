use std::cmp::max;
use std::collections::HashMap;
use itertools::{izip};
use crate::utils::{assert_display, Label, Solve};
use regex::Regex;

#[derive(Debug)]
struct Ingredient{
    capacity: isize,
    durability: isize,
    flavor: isize,
    texture: isize,
    calories: isize
}

pub(crate) struct Advent {
    label: Label,
    ingredients: HashMap<String, Ingredient>
}
impl Default for Advent {
    fn default() -> Self{
        Self{
            label: Label::new(15, 2015),
            ingredients: HashMap::new()
        }
    }
}

impl Advent{
    fn solve(&self,
             calorie_constraint: bool,
             result_prd: isize,
             part: u8
    )->Result<String,String> {
        self.check_input(Some(part))?;
        let ingredients = self.ingredients.iter().map(|(_, v)| v).collect::<Vec<_>>();
        let mut best_score: isize = 0;
        for weights in constrained_vectors(self.ingredients.len(), 100){
            if calorie_constraint {
                if izip!(weights.iter(), ingredients.iter()).map(|(w, i)| i.calories * *w).sum::<isize>() != 500 {
                    continue;
                }
            }
            let mut scores = [0isize; 4];
            for (w, i) in izip!(weights.iter(), ingredients.iter()) {
                scores[0] += i.capacity * *w;
                scores[1] += i.durability * *w;
                scores[2] += i.flavor * *w;
                scores[3] += i.texture * *w;
            }
            let p = scores.iter().map(|s| max(0, *s)).product::<isize>();
            if p > best_score {
                best_score = p;
            }
        }
        assert_display(best_score, None, result_prd, "Best score", false)
    }
}

impl Solve for Advent {

    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn add_record_from_line(&mut self, line: String) -> Result<(), std::num::ParseIntError>{
        let re = Regex::new(r"^(.*): capacity (-?\d+), durability (-?\d+), flavor (-?\d+), texture (-?\d+), calories (-?\d+)$").unwrap();
        if let Some(captures) = re.captures(line.as_str()) {
            let name = String::from(captures.get(1).unwrap().as_str());
            let capacity = captures.get(2).unwrap().as_str().parse::<isize>()?;
            let durability = captures.get(3).unwrap().as_str().parse::<isize>()?;
            let flavor = captures.get(4).unwrap().as_str().parse::<isize>()?;
            let texture = captures.get(5).unwrap().as_str().parse::<isize>()?;
            let calories = captures.get(6).unwrap().as_str().parse::<isize>()?;
            self.ingredients.insert(name, Ingredient{capacity, durability, flavor, texture,calories});
        } else {
            println!("{}", line);
            "invalid".parse::<i32>()?;
        }
        Ok(())
    }

    fn info(&self) -> Result<(), String>{
        self.check_input(None)?;
        println!("Number of ingredients: {}", self.ingredients.len());
        Ok(())
    }
    fn compute_part1_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.solve(false, 18965440,1)
    }
    fn compute_part2_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.solve(true, 15862900,2)
    }
}

fn constrained_vectors(length: usize, sum: isize) -> Vec<Vec<isize>> {
    if length == 1 {
        vec![vec![sum]]
    } else {
        (0..=sum)
            .flat_map(|x| {
                constrained_vectors(length - 1, sum - x)
                    .into_iter()
                    .map(move |v| {
                        let mut new_vec = vec![x];
                        new_vec.extend(v);
                        new_vec
                    })
            })
            .collect()
    }
}