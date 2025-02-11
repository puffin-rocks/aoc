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

type State = (usize, usize, usize, Vec<usize>, Vec<usize>, usize, usize);
//step, |0-G|, |G-E|, used, available, ind(G), ind(E)

impl Solve for Advent {

    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn add_record_from_line(&mut self, line: String) -> Result<(), std::num::ParseIntError>{
        //Filesystem              Size  Used  Avail  Use%
        // if line.starts_with("root") | line.starts_with("Filesystem"){
        //     return Ok(())
        // }

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
        let ind_empty = find_empty(&used);
        let result = iterate((distance(max_x-1, ind_empty, max_x), max_x-1, 0, //( 0, max_x-1, distance(max_x-1, ind_empty, max_x),
                              used, avlb,
                              max_x-1, ind_empty), max_x, max_y);
        println!("{:?}", result);
        Err(String::from("Not implemented"))
    }
}

fn distance(idx1: usize, idx2: usize, max_x: usize)->usize {
    let i1 = (idx1%max_x) as isize;
    let j1 = (idx1/max_x) as isize;
    let i2 = (idx2%max_x) as isize;
    let j2 = (idx2/max_x) as isize;
    ((i1 - i2).abs()  + (j1-j2).abs()) as usize
}

fn find_empty(used: &Vec<usize>)-> usize{
    if used.iter().filter(|v| **v==0).count()>1{
        unreachable!()
    }
    used.iter().position(|v| *v==0).unwrap()
}

fn iterate(state: State, max_x: usize, max_y:usize) -> Option<usize>{
    let mut states: HashSet<(Vec<usize>, Vec<usize>, usize)> = HashSet::new();
    states.insert( (state.3.clone(), state.4.clone(), state.5));
    let mut queue: BTreeSet<State> = BTreeSet::new();
    queue.insert(state);
    let mut cnt = 0;
    let mut dst = 40;
    while let Some((d_e, d_0, step, used, avlb, loc_g, loc_e)) = queue.pop_first() { // step, d_0, d_e
        //println!(" in {:?}", (d_e, d_0, step, loc_g, loc_e));
        let i = (loc_e%max_x) as isize;
        let j = (loc_e/max_x) as isize;
        let a = avlb[loc_e];
        for (i_n, j_n) in [(i-1, j), (i+1, j), (i, j-1), (i, j+1)]{
            if i_n>-1 && i_n<max_x as isize && j_n>-1 && j_n<max_y as isize{
                let idx_n = (i_n + j_n*(max_x as isize)) as usize;
                let to_move = used[idx_n];
                if a>=to_move{
                    let mut used_next = used.clone();
                    let mut avlb_next = avlb.clone();
                    used_next[idx_n] = 0;
                    avlb_next[idx_n]+=to_move;
                    used_next[loc_e]+=to_move;
                    avlb_next[loc_e]-=to_move;

                    let (loc_g_next, d_0_next) = if idx_n == loc_g{
                        (loc_e, (i+j) as usize)
                    }else{
                        (loc_g, d_0)
                    };
                    if loc_g_next == 0{
                        return Some(step+1);
                    }
                    let d_next = distance(loc_g_next, idx_n, max_x);
                    if d_next<dst{
                        println!("{}", d_next);
                        dst = d_next;
                    }
                    let state = (d_next, d_0_next, step+1,  used_next, avlb_next, loc_g_next, idx_n); // step+1, d_0_next, distance(loc_g_next, idx_n, max_x),
                    let substate = (state.3.clone(), state.4.clone(), state.5);
                    if !states.contains(&substate){
                        states.insert(substate);
                        queue.insert(state);
                        cnt+=1;

                        if cnt>10_000 {
                            println!("out {:?}", ((i_n, j_n), d_next, d_0_next, step+1, loc_g_next, idx_n));
                            return None;
                        }
                    }
                }
            }
        }
    }
    None
}