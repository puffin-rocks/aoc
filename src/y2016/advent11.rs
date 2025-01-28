use std::collections::{HashMap, HashSet};
use regex::Regex;
use crate::hashset;
use crate::utils::{assert_display, Label, Solve};

#[derive(Debug, Eq, Hash, PartialEq, Ord, PartialOrd, Clone)]
enum Component{
    Generator(char),
    Microchip(char)
}


pub(crate) struct Advent {
    label: Label,
    components: HashSet<(usize, Component)>,
    n_floors: usize
}
impl Default for Advent {
    fn default() -> Self{
        Self{
            label: Label::new(11, 2016),
            components: HashSet::new(),
            n_floors: 0
        }
    }
}

impl Solve for Advent {

    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn add_record_from_line(&mut self, line: String) -> Result<(), std::num::ParseIntError>{

        let re = Regex::new(r"(?m)^The (\w+) floor contains (.+?)\.").unwrap();

        for cap in re.captures_iter(line.as_str()) {
            let floor = match &cap[1] {
                "first" => 0,
                "second" => 1,
                "third" => 2,
                "fourth" => 3,
                _ => continue,
            };

            let re_item = Regex::new(r"(\w+) generator").unwrap();
            for item_cap in re_item.captures_iter(&cap[2]) {
                self.components.insert((floor, Component::Generator(item_cap[0].chars().nth(0).unwrap())));
            }

            let re_item = Regex::new(r"(\w+)-compatible microchip").unwrap();
            for item_cap in re_item.captures_iter(&cap[2]) {
                self.components.insert((floor, Component::Microchip(item_cap[0].chars().nth(0).unwrap())));
            }
            if floor>self.n_floors{
                self.n_floors=floor+1;
            }
        }
        Ok(())
    }

    fn info(&self) -> Result<(), String>{
        self.check_input(None)?;
        println!("Components {:?}", self.components);
        Ok(())
    }
    fn compute_part1_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.check_input(Some(1))?;
        let mut components = self.components.clone();
        //part 2 answer is 61, current method super slow, probably need recursion
        components.insert((0, Component::Generator('e')));
        components.insert((0, Component::Generator('d')));
        components.insert((0, Component::Microchip('e')));
        components.insert((0, Component::Microchip('d')));

        let mut curr_state: Vec<Vec<Component>> = vec![Vec::new(); self.n_floors];
        for (floor, component) in components.iter(){
            curr_state.get_mut(*floor).unwrap().push(component.clone());
        }
        for set in curr_state.iter_mut(){
            set.sort_unstable();
        }
        let curr_floor = 0usize;
        let mut curr_step = 0usize;
        let mut states: HashMap<(usize, Vec<Vec<Component>>), usize> = HashMap::new();
        states.insert((curr_floor, curr_state.clone()), curr_step);

        let mut queue = hashset!((curr_floor, curr_state));
        let mut result: Option<usize> = None;
        loop {
            let mut queue_next: HashSet<(usize, Vec<Vec<Component>>)> = HashSet::new();
            let mut found_end_state = false;
            for (curr_floor, curr_state) in queue.iter(){
                let next = next_states(curr_state, *curr_floor, self.n_floors, curr_step, &mut states);
                for (next_floor, s) in next{
                    if end_condition(&s, self.n_floors){
                        found_end_state = true;
                        result = Some(curr_step+1);
                        break;
                    }else{
                        queue_next.insert((next_floor, s));
                    }
                }
                if found_end_state{
                    println!("ES{}", curr_step+1);
                    break;
                }
            }
            queue = queue_next;
            if queue.len() ==0{
                break;
            }
            println!("QL{}", queue.len());
            curr_step+=1;
        }
        match result {
            None => Err(String::from("Not solution found")),
            Some(n_steps) => assert_display(n_steps, None,37, "Minimal number of steps", false)
        }
    }
    fn compute_part2_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.check_input(Some(2))?;
        Err(String::from("Not implemented"))
    }
}

fn end_condition(curr_state: &Vec<Vec<Component>>, n_floors: usize) ->bool{
    for ix in 0..n_floors-1{
        if !curr_state.get(ix).unwrap().is_empty(){
            return false;
        }
    }
    true
}

fn next_states(curr_state: &Vec<Vec<Component>>, curr_floor: usize, n_floors: usize, curr_step: usize, states: &mut HashMap<(usize, Vec<Vec<Component>>), usize>)->HashSet<(usize, Vec<Vec<Component>>)>{
    let components = curr_state.get(curr_floor).unwrap();
    let mut next_floors: Vec<usize> = Vec::new();
    for x in [-1isize, 1isize]{
        let next_floor =  curr_floor as isize + x;
        if next_floor>-1 && next_floor < n_floors as isize{
            next_floors.push(next_floor as usize);
        }
    }
    let mut result: HashSet<(usize, Vec<Vec<Component>>)> = HashSet::new();

    let mut process_state = |next_state: Vec<Vec<Component>>, next_floor: usize, result: &mut HashSet<(usize, Vec<Vec<Component>>)>| {
        if check_state(&next_state) {
            if states.get(&(next_floor, next_state.clone())).map_or(true, |&n| curr_step + 1 < n) {
                states.insert((next_floor, next_state.clone()), curr_step + 1);
                result.insert((next_floor, next_state));
            }
        }
    };

    for ix in 0..components.len(){
        for next_floor in next_floors.iter(){
            let mut next_state = curr_state.clone();
            let el = next_state.get_mut(curr_floor).unwrap().remove(ix);
            let floor_components = next_state.get_mut(*next_floor).unwrap();
            floor_components.push(el);
            floor_components.sort_unstable();
            process_state(next_state, *next_floor, &mut result);
        }
    }
    if components.len()>1 {
        for i in 0..components.len() - 1 {
            for j in i + 1..components.len() {
                for next_floor in next_floors.iter() {
                    let mut next_state = curr_state.clone();
                    for ix in [j, i] {
                        let el = next_state.get_mut(curr_floor).unwrap().remove(ix);
                        let floor_components = next_state.get_mut(*next_floor).unwrap();
                        floor_components.push(el);
                    }
                    let floor_components = next_state.get_mut(*next_floor).unwrap();
                    floor_components.sort_unstable();
                    process_state(next_state, *next_floor, &mut result);
                }
            }
        }
    }
    result
}

fn check_state(curr_state: &Vec<Vec<Component>>)->bool{
    for floor_components in curr_state{
        let mut generators: Vec<&char> = Vec::new();
        let mut microchips: Vec<&char> = Vec::new();
        for el in floor_components{
            match el {
                Component::Generator(ch) => generators.push(ch),
                Component::Microchip(ch) => microchips.push(ch),
            }
        }
        if generators.is_empty(){
            continue;
        }
        for m in microchips.iter(){
            let mut orphan = true;
            for g in generators.iter(){
                if m==g{
                    orphan = false;
                    break;
                }
            }
            if orphan{
                //println!("{:?}", (m, floor_components));
                return false;
            }
        }
    }
    true
}