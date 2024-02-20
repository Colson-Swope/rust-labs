use std::collections::HashSet;
use std::io::{self, BufRead};
use std::fs::File;

fn set_logic(a: HashSet<Vec<String>>, b: HashSet<Vec<String>>) {
    println!();
    // Display set A
    println!("Set A:");
    println!();
    for item in &a {
        println!("{:?}", item);
    }
    
    println!();
    // Display set B
    println!("Set B:");
    println!();
    for item in &b {
        println!("{:?}", item);
    }
    
    println!();
    // ∪ 
    println!("Union (Set A ∪ Set B):");
    println!();
    for item in a.union(&b) {
        println!("{:?}", item);
    }

    println!();
    // \ 
    println!("Difference (Set A \\ Set B):");
    println!();
    for item in a.difference(&b) {
        println!("{:?}", item);
    }

    println!();
    // ∩
    println!("Intersection (Set A ∩ Set B):");
    println!();
    for item in a.intersection(&b) {
        println!("{:?}", item);
    }

    println!();
    // Δ
    println!("Symmetric Difference (Set A Δ Set B):");
    println!();
    for item in a.symmetric_difference(&b) {
        println!("{:?}", item);
    }
    println!();
}

// to implement this, use documentation + replicate key_input 
fn file_input(user_file: &str) -> HashSet<Vec<String>> {

    let passed_file = File::open(user_file).expect("Error 3: Could not open file."); 
    let reader = io::BufReader::new(passed_file); 

    let mut table = HashSet::new(); 

    for line in reader.lines() {
        let row: Vec<String> = line.expect("Error 4: Could not read line").trim().split_whitespace().map(|s| s.to_string()).collect(); 
        table.insert(row); 
    }
    table
}

fn key_input() -> HashSet<Vec<String>> {

    println!("Enter values for table (one row per line, space-separated)");

    let mut table = HashSet::new();
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        
        let row: Vec<String> = line.unwrap().trim().split_whitespace().map(|s| s.to_string()).collect();
        
        if row.is_empty() {
            break;
        }

        table.insert(row);
    }
    
    table
	
}

fn main() {

    loop {

        println!("Enter a choice: "); 
        println!("1. Enter input from keyboard");
        println!("2. Use file for input"); 
        println!("3. Exit");

        let mut user_choice = String::new(); 

        io::stdin().read_line(&mut user_choice).expect("Error 5: Failed to read line"); 

        // process input, check for error 
        let num: i32 = match user_choice.trim().parse() {
            Ok(num) => num, 
            Err(_) => {
                println!("Error 1: Please enter a numerical value.");
                println!();
                continue; 
            }
        };

        // check if 1 or 2 or 3
        if num == 1 {

            let table_a = key_input();
            let table_b = key_input();
            set_logic(table_a, table_b);

        } 
        else if num == 2 {
            
            println!("Enter a file for Table A");
            
            let mut user_path_table_a = String::new(); 

            io::stdin().read_line(&mut user_path_table_a).expect("Error 6: Failed to read line for table A"); 

            // Remove newline character because it is causing issues with the file path 
            user_path_table_a = user_path_table_a.trim().to_string();


            println!("Enter a file for Table B");
            
            let mut user_path_table_b = String::new(); 

            io::stdin().read_line(&mut user_path_table_b).expect("Error 7: Failed to read line for table B"); 

            // Remove newline character because it is causing issues with the file path 
            user_path_table_b = user_path_table_b.trim().to_string();

            let table_a = file_input(&user_path_table_a);
            let table_b = file_input(&user_path_table_b);
            set_logic(table_a, table_b);

        }
        else if num == 3 {

            return; 

        }
        else {
            println!("Error 2: Please enter a value between 1 and 3."); 
            println!();
        }
    }
}
