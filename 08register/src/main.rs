extern crate utilities;
use std::collections::HashMap;

fn main() {
    let input = utilities::read_file("input.txt");

    let instruction_list = create_instruction_list(&input);
    let registers = execute_instructions(&instruction_list);
    println!("{:?}", registers.values().max().unwrap());
}

fn execute_instructions(instructions: &Vec<RegisterInstruction>) -> HashMap<String, i32> {
    let mut registers = HashMap::new();
    let mut max_value = i32::min_value();

    for instruct in instructions {
        // Find out if affected register already exists
        if !registers.contains_key(&instruct.register_affected) {
            registers.insert(instruct.register_affected.clone(), 0);
        }

        if !registers.contains_key(&instruct.if_register) {
            registers.insert(instruct.if_register.clone(), 0);
        }

        // Get value for if register
        let if_register_amount: i32 = *registers.get(&instruct.if_register).unwrap() as i32;

        // Evaluate if statement
        let if_condition_result = match instruct.if_condition {
            IfCondition::Gt  => if_register_amount > instruct.if_amount,
            IfCondition::Gte => if_register_amount >= instruct.if_amount,
            IfCondition::Lt  => if_register_amount < instruct.if_amount,
            IfCondition::Lte => if_register_amount <= instruct.if_amount,
            IfCondition::Eq  => if_register_amount == instruct.if_amount,
            IfCondition::Neq => if_register_amount != instruct.if_amount,
        };

        // If true, change affected register appropriately
        if if_condition_result {
            let mut num: i32 = *registers.get(&instruct.register_affected).unwrap() as i32;
            num = match instruct.command {
                Command::Dec => num - instruct.change_amount,
                Command::Inc => num + instruct.change_amount,
            };
            if num > max_value {
                max_value = num;
            }
            registers.insert(instruct.register_affected.clone(), num);
        }
    }

    println!("Highest value ever held in a register: {}", max_value);

    registers
}

#[derive(Debug)]
struct RegisterInstruction {
    register_affected: String,
    command: Command,
    change_amount: i32,
    if_register: String,
    if_condition: IfCondition,
    if_amount: i32,
}

#[derive(Debug)]
enum Command {
    Inc,
    Dec,
}

#[derive(Debug)]
enum IfCondition {
    Gt,
    Gte,
    Lt,
    Lte,
    Eq,
    Neq,
}

fn create_instruction_list(input: &String) -> Vec<RegisterInstruction> {
    let mut result = Vec::new();
    for instruction in input.lines() {

        let mut parts = instruction.split(" ");

        let register_affected = parts.next().unwrap().to_owned();

        let command = match parts.next().unwrap() {
            "inc" => Command::Inc,
            _ => Command::Dec,
        };

        let change_amount = parts.next().unwrap().parse::<i32>().unwrap();

        parts.next(); // Get rid of if
        let if_register = parts.next().unwrap().to_owned();

        let if_condition = match parts.next().unwrap() {
            ">"  => IfCondition::Gt,
            ">=" => IfCondition::Gte,
            "<"  => IfCondition::Lt,
            "<=" => IfCondition::Lte,
            "==" => IfCondition::Eq,
            _    => IfCondition::Neq,
        };

        let if_amount = parts.next().unwrap().parse::<i32>().unwrap();

        let instruction = RegisterInstruction {
            register_affected,
            command,
            change_amount,
            if_register,
            if_condition,
            if_amount,
        };
        result.push(instruction);
    }
    result
}
