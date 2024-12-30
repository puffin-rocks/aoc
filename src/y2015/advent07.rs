use crate::utils::{assert_display, Label, Solve};
use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone)]
enum Value{
    Address(String),
    Integer(u16),
    None
}

#[derive(Debug, Clone, Copy, Hash, PartialEq)]
enum Operator{
    AND,
    OR,
    RSHIFT,
    LSHIFT,
    NOT,
    SELF
}

impl Eq for Operator {}
#[derive(Debug, Clone)]
struct Gate{
    input1: Value,
    input2: Value,
    output: String,
    operator: Operator
}

impl Gate{
    fn new(input1: Value, input2: Value, output: String, operator: Operator)->Self{
        Self{
            input1,
            input2,
            output,
            operator
        }
    }
    fn one_arg_operation(&self, number: u16, values: &mut HashMap<String,u16>){
        let value = match self.operator {
            Operator::SELF => number,
            Operator::NOT => !number,
            _ => unreachable!()
        };
        values.insert(self.output.clone(), value);
    }
    fn two_arg_operation(&self, number1: u16, number2: u16,  values: &mut HashMap<String,u16>){
        let value = match self.operator {
            Operator::AND => number1 & number2,
            Operator::OR => number1 | number2,
            Operator::RSHIFT => number1 >> number2,
            Operator::LSHIFT => number1 << number2,
            _ => unreachable!()
        };
        values.insert(self.output.clone(), value);
    }
    fn evaluate(&self, values: &mut HashMap<String,u16>)->Result<(),()>{
        match (&self.input1, &self.input2){
            (Value::Address(key1), Value::Address(key2)) => {
                if let (Some(number1), Some(number2)) =
                    (values.get(key1), values.get(key2)){
                    self.two_arg_operation(*number1, *number2, values);
                    Ok(())
                }else{
                    Err(())
                }
            },
            (Value::Address(key1), Value::Integer(number2)) => {
                if let Some(number1) = values.get(key1){
                    self.two_arg_operation(*number1, *number2, values);
                    Ok(())
                }else{
                    Err(())
                }
            },
            (Value::Address(key1), Value::None) => {
                if let Some(number1) = values.get(key1){
                    self.one_arg_operation(*number1, values);
                    Ok(())
                }else{
                    Err(())
                }
            },
            (Value::Integer(number1), Value::Address(key2)) => {
                if let Some(number2) = values.get(key2){
                    self.two_arg_operation(*number1, *number2, values);
                    Ok(())
                }else{
                    Err(())
                }
            },
            (Value::Integer(number1), Value::Integer(number2)) => {
                self.two_arg_operation(*number1, *number2, values);
                Ok(())
            },
            (Value::Integer(number1), Value::None) => {
                self.one_arg_operation(*number1, values);
                Ok(())
            },
            _ => unreachable!()
        }
    }
}

pub(crate) struct Advent {
    label: Label,
    values: HashMap<String, u16>,
    gates: Vec<Gate>,
}
impl Default for Advent {
    fn default() -> Self{
        Self{
            label: Label::new(7, 2015),
            values: HashMap::new(),
            gates: Vec::new()
        }
    }
}

impl Solve for Advent {

    fn get_label(&self) -> &Label{ &self.label }
    fn get_label_mut(&mut self) -> &mut Label {&mut self.label}

    fn add_record_from_line(&mut self, line: String) -> Result<(), std::num::ParseIntError>{
        if let Some((gate_def,out)) = line.split_once(" -> "){
            let out = out.to_string();
            let parts = gate_def.split(" ").collect::<Vec<_>>();
            let n = parts.len();
            let g: Gate = match n{
                3 =>{
                    let operator = match parts[1]{
                        "AND" => Operator::AND,
                        "OR" => Operator::OR,
                        "LSHIFT" => Operator::LSHIFT,
                        "RSHIFT" => Operator::RSHIFT,
                        _ => unreachable!()
                    };
                    match (parts[0].parse::<u16>(), parts[2].parse::<u16>()){
                        (Ok(number1), Ok(number2)) => Gate::new(Value::Integer(number1),
                        Value::Integer(number2), out, operator),
                        (Ok(number1), Err(_)) => Gate::new(Value::Integer(number1),
                                                                    Value::Address(parts[2].to_string()), out, operator),
                        (Err(_), Ok(number2)) => Gate::new(Value::Address(parts[0].to_string()),
                                                                    Value::Integer(number2), out, operator),
                        (Err(_), Err(_)) => Gate::new(Value::Address(parts[0].to_string()),
                                                  Value::Address(parts[2].to_string()), out, operator)
                    }
                },
                2 =>{
                    assert_eq!(parts[0], "NOT");
                    match parts[1].parse::<u16>(){
                        Ok(number) =>
                            Gate::new(Value::Integer(number),
                                      Value::None, out, Operator::NOT),
                        Err(_) => Gate::new(Value::Address(parts[1].to_string()),
                                          Value::None, out, Operator::NOT)
                    }
                },
                1 =>{
                    match parts[0].parse::<u16>(){
                        Ok(number) =>
                            Gate::new(Value::Integer(number),
                                      Value::None, out, Operator::SELF),
                        Err(_) => Gate::new(Value::Address(parts[0].to_string()),
                        Value::None, out, Operator::SELF)
                    }
                }
                _ => unreachable!()
            };
            self.gates.push(g);
        };
        Ok(())
    }

    fn info(&self) -> Result<(), String>{
        self.check_input(None)?;
        println!("Number of gates: {}", self.gates.len());
        Ok(())
    }

    fn compute_part1_answer(&self, _test_mode: bool) -> Result<String, String>{
        self.check_input(Some(1))?;
        let mut values = self.values.clone();
        let mut queue: VecDeque<Gate> = VecDeque::new();
        queue.extend(self.gates.iter().cloned());
        let result = get_wire_a(queue, &mut values)?;
        assert_display(result, None, 16076, "Value of wire a", false)
    }

    fn compute_part2_answer(&self,  _: bool) -> Result<String, String>{
        self.check_input(Some(2))?;
        let mut values = self.values.clone();
        let mut queue: VecDeque<Gate> = VecDeque::new();
        queue.extend(self.gates.iter().cloned());
        let wire_a = get_wire_a(queue, &mut values)?;

        let mut values = self.values.clone();
        let mut queue: VecDeque<Gate> = VecDeque::new();
        for g in self.gates.iter(){
            if g.output =="b"{
                let mut g = g.clone();
                g.input1 = Value::Integer(wire_a);
                queue.push_back(g);
            }else{
                queue.push_back(g.clone());
            }
        }
        let result = get_wire_a(queue, &mut values)?;
        assert_display(result, None, 2797, "Value of wire a", false)
    }
}

fn get_wire_a(mut queue: VecDeque<Gate>, values: &mut HashMap<String,u16>)->Result<u16, String>{
    while !queue.is_empty(){
        if let Some(g) = queue.pop_front(){
            match g.evaluate(values){
                Err(_) => {
                    queue.push_back(g);
                }
                Ok(_) => {
                    if g.output=="a"{
                        return Ok(*values.get("a").unwrap());
                    }
                }
            }
        }
    }
    Err(String::from("Wire a is not reached"))
}