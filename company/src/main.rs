use std::collections::HashMap;
use std::io;

fn list_commands() {
    println!("-----------Company-----------");
    println!("Commands");
    println!("  Adding an Employee:");
    println!("  > add Sally to Engineering");
    println!("  Listing Departments:");
    println!("  > departments");
    println!("  Listing Employees:");
    println!("  > Engineering");
    println!("  Listing Commands:");
    println!("  > help");
    println!("-----------------------------");
}

fn list_departments(company: &HashMap<String, Vec<String>>) {
    if company.len() > 0 {
        println!("\nDepartments:");
        for department in company.keys() {
            println!("{}", department);
        }
    } else {
        println!("There are no departments yet.");
    }
}

fn main() {
    let mut company: HashMap<String, Vec<String>> = HashMap::new();
    list_commands();

    loop {
        println!("");
        println!("Enter your command.");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read the line");

        let split = input.trim().split(" ");
        let vec = split.collect::<Vec<&str>>();
        if vec.len() == 4 {
            if vec[0].eq("add") && vec[2].eq("to") {
                company
                    .entry(vec[3].to_string())
                    .or_insert(Vec::<String>::new())
                    .push(vec[1].to_string());
            }
        } else if vec.len() == 1 {
            if vec[0].eq("departments") {
                list_departments(&company);
            } else if company.contains_key(&vec[0].to_string()) {
                println!("\nEmployees:");
                println!("{:?}", company.get(vec[0]).unwrap());
            } else if vec[0].eq("help") {
                list_commands();
            }
        }
    }
}
