use std::collections::{BTreeSet, HashMap, HashSet};
use regex::Regex;
use crate::utils::{assert_display, Label, Solve};

pub(crate) struct Advent {
    label: Label,
    grid: HashMap<(usize,usize),(usize,usize)>
}
impl Default for Advent {
    fn default() -> Self{
        Self{
            label: Label::new(22, 2016),
            grid: HashMap::new()
        }
    }
}

impl Solve for Advent {

    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn add_record_from_line(&mut self, line: String) -> Result<(), std::num::ParseIntError>{
        let re = Regex::new(r"/dev/grid/node-x(\d+)-y(\d+)\s+(\d+)T\s+(\d+)T\s+(\d+)T\s+(\d+)%").unwrap();
        if let Some(captures) = re.captures(line.as_str()){
            let x = captures.get(1).unwrap().as_str().parse::<usize>()?;
            let y = captures.get(2).unwrap().as_str().parse::<usize>()?;
            let used = captures.get(4).unwrap().as_str().parse::<usize>()?;
            let avail = captures.get(5).unwrap().as_str().parse::<usize>()?;
            self.grid.insert((x,y), (used, avail));
        }
        Ok(())
    }

    fn info(&self) -> Result<(), String>{
        self.check_input(None)?;
        println!("Number of nodes: {}",self.grid.len());
        Ok(())
    }
    fn compute_part1_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.check_input(Some(1))?;
        let n = self.grid.iter()
            .flat_map(|(k1, v1)| {
                self.grid.iter().filter(move |(k2, v2)| {
                    k1 != *k2 && v1.0 > 0 && v1.0 <= v2.1
                })
            }).count();
        assert_display(n, None, 903, "Number of viable pairs", false)
    }
    fn compute_part2_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.check_input(Some(2))?;
        let max_x = self.grid.iter().map(|(k,_)| k.0).max().unwrap()+1;
        let max_y = self.grid.iter().map(|(k,_)| k.1).max().unwrap()+1;
        let n =max_x*max_y;
        let mut used: Vec<usize> = vec![0; n];
        let mut avlb: Vec<usize> = vec![0; n];
        for j in 0..max_y{
            for i in 0..max_x{
                let idx = i + j*max_x;
                let (u, a ) = self.grid.get(&(i,j)).unwrap();
                used[idx]=*u;
                avlb[idx]=*a;
            }
        }
        //checking that separation holds
        if used.iter().filter(|v| **v==0).count()>1{
            return Err(String::from("Not solution found"));
        }
        let ind_empty = used.iter().position(|v| *v==0);
        if ind_empty.is_none(){
            return Err(String::from("Not solution found"));
        }
        let ind_empty = ind_empty.unwrap();
        let capacity_empty= avlb[ind_empty];
        let mut capacity_small:Vec<usize> = Vec::new();
        let mut used_small:Vec<usize> = Vec::new();
        let mut avlb_large:Vec<usize> = Vec::new();
        let mut used_large:Vec<usize> = Vec::new();
        for i in 0..n{
            if used[i]>0 {
                if used[i] <= capacity_empty {
                    capacity_small.push(avlb[i]+used[i]);
                    used_small.push(used[i]);
                } else {
                    avlb_large.push(avlb[i]);
                    used_large.push(used[i]);
                }
            }
        }

        assert_eq!(capacity_small.iter().min().unwrap()>used_small.iter().max().unwrap(), true);
        assert_eq!(used_large.iter().min().unwrap()>capacity_small.iter().max().unwrap(), true);
        assert_eq!(avlb_large.iter().max().unwrap()<used_small.iter().min().unwrap(), true);

        let mut blocks: HashSet<usize> = HashSet::new();
        for i in 0..n{
            if i != max_x-1 && used[i] > 0 && used[i] > capacity_empty{
                blocks.insert(i);
            }
        }

        let result = iterate(ind_empty, max_x-1, blocks, max_x, max_y);
        match result{
            None => Err(String::from("Not solution found")),
            Some(v) => assert_display(v, None, 215, "Min number of steps", false)
        }

    }
}

fn _iterate(loc_e: usize, loc_g: usize, blocks: HashSet<usize>, max_x: usize, max_y:usize) -> Option<usize>{
    fn distance(idx1: usize, idx2: usize, max_x: usize)->usize {
        let i1 = (idx1%max_x) as isize;
        let j1 = (idx1/max_x) as isize;
        let i2 = (idx2%max_x) as isize;
        let j2 = (idx2/max_x) as isize;
        ((i1 - i2).abs()  + (j1-j2).abs()) as usize
    }

    //this solution is slightly slower
    let mut states: HashMap<(usize, usize), usize> = HashMap::new();
    states.insert( (loc_e, loc_g), 0);
    let mut queue: BTreeSet<(usize, usize, usize, usize, usize)> = BTreeSet::new(); //|E-G|, |G-0|, step, symbols, loc E, loc G
    queue.insert((distance(loc_g, loc_e, max_x), max_x-1, 0, loc_e, loc_g));
    let mut solution: Option<usize> = None;
    while let Some((_, _, step, loc_e, loc_g)) = queue.pop_first(){
        if solution.unwrap_or(usize::MAX-1)+1<=step{
            continue;
        }
        let i = (loc_e%max_x) as isize;
        let j = (loc_e/max_x) as isize;
        for (i_n, j_n) in [(i-1, j), (i+1, j), (i, j-1), (i, j+1)] {
            if i_n > -1 && i_n < max_x as isize && j_n > -1 && j_n < max_y as isize {
                let idx_n = (i_n + j_n * (max_x as isize)) as usize;
                if loc_e == 0 && idx_n == loc_g{
                    match solution {
                        None => {
                            solution = Some(step+1);
                        },
                        Some(v)=>{
                            if v>step+1{
                                solution = Some(step+1);
                            }
                        }
                    }
                }
                if !blocks.contains(&idx_n){
                    let loc_g_next = if idx_n == loc_g{
                        loc_e
                    }else{
                        loc_g
                    };
                    let state_next = (idx_n, loc_g_next);
                    if states.get(&state_next).unwrap_or(&usize::MAX)>&(step+1){
                        states.insert(state_next, step+1);
                        queue.insert(( distance(idx_n, loc_g_next, max_x), distance(loc_g_next, 0, max_x), step+1, idx_n, loc_g_next));
                    }
                }
            }
        }
    }
    solution
}

fn iterate(loc_e: usize, loc_g: usize, blocks: HashSet<usize>, max_x: usize, max_y:usize) -> Option<usize>{
    let mut states: HashSet<(usize, usize)> = HashSet::new();
    states.insert( (loc_e, loc_g));
    let mut queue: Vec<(usize, usize)> = Vec::new();
    queue.push((loc_e, loc_g));
    let mut step = 0;
    loop{
        let mut queue_next: Vec<(usize, usize)> = Vec::new();
        for (loc_e, loc_g) in queue.iter(){
            let i = (*loc_e%max_x) as isize;
            let j = (*loc_e/max_x) as isize;
            for (i_n, j_n) in [(i-1, j), (i+1, j), (i, j-1), (i, j+1)] {
                if i_n > -1 && i_n < max_x as isize && j_n > -1 && j_n < max_y as isize {
                    let idx_n = (i_n + j_n * (max_x as isize)) as usize;
                    if *loc_e == 0 && idx_n == *loc_g{
                        return Some(step+1);
                    }
                    if !blocks.contains(&idx_n){
                        let loc_g_next = if idx_n == *loc_g{
                            loc_e
                        }else{
                            loc_g
                        };
                        let state_next = (idx_n, *loc_g_next);
                        if !states.contains(&state_next){
                            states.insert(state_next);
                            queue_next.push(state_next);
                        }
                    }
                }
            }
        }
        if queue_next.is_empty(){
            return None;
        }
        queue = queue_next;
        step+=1;
    }
}