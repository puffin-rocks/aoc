use crate::utils::{assert_display, swap_vec_elements, Label, Solve};

pub(crate) struct Advent {
    label: Label,
    moves: Vec<String>
}
impl Default for Advent {
    fn default() -> Self{
        Self{
            label: Label::new(16, 2017),
            moves: Vec::new()
        }
    }
}

impl Advent{
    fn dance(&self, order: &mut String) {
        let n_order = order.len();
        for m in self.moves.iter() {
            let ch = m.chars().nth(0).unwrap();
            match ch {
                's' => {
                    let n = m[1..].parse::<usize>().expect("Cannot convert string");
                    let (left, right) = order.split_at_mut(n_order - n);
                    let mut order_next = String::from(right);
                    order_next.extend(left.chars());
                    *order = order_next;
                },
                'x' => {
                    if let Some((v1, v2)) = m[1..].split_once("/") {
                        let p1 = v1.parse::<usize>().expect("Cannot convert string");
                        let p2 = v2.parse::<usize>().expect("Cannot convert string");
                        let mut order_next = order.chars().clone().collect::<Vec<char>>();
                        swap_vec_elements(&mut order_next, p1, p2);
                        *order = order_next.into_iter().collect::<String>();
                    }
                },
                'p' => {
                    if let Some((v1, v2)) = m[1..].split_once("/") {
                        let ch1 = v1.chars().nth(0).unwrap();
                        let ch2 = v2.chars().nth(0).unwrap();
                        let p1 = order.find(ch1).unwrap();
                        let p2 = order.find(ch2).unwrap();
                        let mut order_next = order.chars().clone().collect::<Vec<char>>();
                        swap_vec_elements(&mut order_next, p1, p2);
                        *order = order_next.into_iter().collect::<String>();
                    }
                },
                _ => unreachable!()
            };

        }
    }
}

impl Solve for Advent {

    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn add_record_from_line(&mut self, line: String) -> Result<(), std::num::ParseIntError>{
        self.moves = line.split(",").map(|x| String::from(x)).collect();
        Ok(())
    }

    fn info(&self) -> Result<(), String>{
        self.check_input(None)?;
        println!("Number of moves: {}", self.moves.len());
        Ok(())
    }
    fn compute_part1_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.check_input(Some(1))?;
        let mut order = ('a'..='p').collect::<String>();
        self.dance(&mut order);
        assert_display(order.as_str(), None, "giadhmkpcnbfjelo", "Final order", false)
    }
    fn compute_part2_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.check_input(Some(2))?;
        let mut order = ('a'..='p').collect::<String>();
        let mut path = vec![order.clone()];
        let mut cnt = 0;
        loop{
            self.dance(&mut order);
            cnt+=1;
            if path.contains(&order){
                break;
            }
            path.push(order.clone());
        }
        let loc = path.iter().position(|x| x==&order).unwrap();
        let rest = (1_000_000_000 - loc)%(cnt-loc);
        let mut order = path[loc].clone();
        for _ in 0..rest{
            self.dance(&mut order);
        }
        assert_display(order.as_str(), None, "njfgilbkcoemhpad", "Final order", false)
    }
}
