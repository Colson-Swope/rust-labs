use regex::Regex;
use std::io; 
use std::io::Read; 
use std::fs::File;

// https://rust-lang-nursery.github.io/rust-cookbook/text/regex.html


fn text_search(user_file_path: &str) {

    // set our passed file to the associated variable 
    let mut passed_file = File::open(user_file_path).expect("Could not open file");

    // read file content into string 
    let mut file_contents = String::new();
    passed_file.read_to_string(&mut file_contents).expect("Could not read file");

    // set regex pattern 
    // address assumed format: street number, Street Name (space separated), unit number 
    let re = Regex::new(r"([A-Za-z]+ [A-Za-z]+), (\d+) ([A-Za-z]+ [A-Za-z]+), (Apt|Suite|Lot|Floor|Unit|House) (\d+), ([A-Za-z]+), (Female|Male), (\d+)").expect("Invalid regex pattern");

    // iterate over matching content from the file data 
    for cap in re.captures_iter(&file_contents) {
        println!(" {}  {}  {} {} ", &cap[2], &cap[3], &cap[4], &cap[5]);
    }

}

fn log_search(user_log_path: &str) {

    // set our passed file to the associated variable 
    let mut passed_file = File::open(user_log_path).expect("Could not open file");

    // read file content into string 
    let mut file_contents = String::new();
    passed_file.read_to_string(&mut file_contents).expect("Could not read file");

    // set regex pattern 
    // log flagged format: "Who has" to look for arp broadcasts 
    let re = Regex::new(r"(Who has .+)").expect("Invalid regex pattern");

    // iterate over matching content from the file data 
    for cap in re.captures_iter(&file_contents) {
        println!(" {} ", &cap[1]);
    }

}

fn na_phone_search(user_number_path: &str) {

    let mut passed_file = File::open(user_number_path).expect("Could not open file"); 

    // read file content into string 
    let mut file_contents = String::new();
    passed_file.read_to_string(&mut file_contents).expect("Could not read file");

    let re_nanp = Regex::new(r#"(?x)
    \b((?:[2-9][0-9]{2})\s*[-/\\.]?)?     # area code 
    ([2-9][0-9]{2})\s*[-/\\.]? # exchange code 
    ([0-9]{4})\b              # subscriber number? 
    "#).expect("Invalid regex pattern for NANP numbers");

    for cap in re_nanp.captures_iter(&file_contents) {
        println!(" {} ", &cap[0]);
    }

}

fn ot_phone_search(user_number_path: &str) {

    let mut passed_file = File::open(user_number_path).expect("Could not open file"); 

    // read file content into string 
    let mut file_contents = String::new();
    passed_file.read_to_string(&mut file_contents).expect("Could not read file");

    let re_international = Regex::new(r#"(?x)
    (\+|\b00)?         # Optional '+' or '00' indicating international format
    (\d{1,3}\s)?       # Optional country code followed by whitespace
    (\d{1,2}\s)?       # Optional area code followed by whitespace
    (\d{1,4}\s)?       # Optional city or region code followed by whitespace
    (\d{4,10})         # Subscriber number
    "#).expect("Invalid regex pattern for international numbers");

    for cap in re_international.captures_iter(&file_contents) {
        println!(" {} ", &cap[0]);
    }

}

fn main() {

    loop {
        // start screen, gives user list of options to choose from 
        println!("Enter a choice: "); 
        println!("1. Search text files looking for addresses"); 
        println!("2. Search network log files"); 
        println!("3. Find and format phone numbers"); 
        println!("4. Exit"); 
        println!();

        let mut user_choice = String::new(); 
        io::stdin().read_line(&mut user_choice).expect("Failed to read line"); 

        // process input, check for error 
        let num: i32 = match user_choice.trim().parse() {
            Ok(num) => num, 
            Err(_) => {
                println!("Please enter a numerical value"); 
                continue; 
            }
        };

        println!();

        if num == 1 {
            println!("Enter a file to search: "); 

            println!(); 

            let mut user_file_path = String::new(); 
            io::stdin().read_line(&mut user_file_path).expect("Failed to read provided file path"); 
            // remove new line character 
            user_file_path = user_file_path.trim().to_string(); 

            println!();

            text_search(&user_file_path);

            println!();
        }

        if num == 2 {
            println!("Enter a log file to search: "); 

            println!();

            let mut user_log_path = String::new(); 
            io::stdin().read_line(&mut user_log_path).expect("Failed to read provided file path"); 
            user_log_path = user_log_path.trim().to_string(); 

            println!();

            log_search(&user_log_path); 

            println!();
        }

        if num == 3 {
            println!("Are you entering a North American or International number log file?");
            println!("1. North American"); 
            println!("2. International");  

            println!(); 

            let mut user_format_selection = String::new(); 
            io::stdin().read_line(&mut user_format_selection).expect("Failed to read provided selection"); 
            user_format_selection = user_format_selection.trim().to_string(); 

            // process input, check for error 
            let num_2: i32 = match user_format_selection.trim().parse() {
                Ok(num_2) => num_2, 
                Err(_) => {
                println!("Please enter a numerical value"); 
                continue; 
            }
            };

            println!(); 

            if num_2 == 1 {
                println!("Enter a North American phone number file to search: "); 

                let mut user_number_path = String::new(); 
                io::stdin().read_line(&mut user_number_path).expect("Failed to read provided file input"); 
                user_number_path = user_number_path.trim().to_string(); 

                na_phone_search(&user_number_path); 
            }
            if num_2 == 2 {
                println!("Enter an International phone number file to search: "); 

                let mut user_number_path = String::new(); 
                io::stdin().read_line(&mut user_number_path).expect("Failed to read provided file input"); 
                user_number_path = user_number_path.trim().to_string(); 

                ot_phone_search(&user_number_path); 
            }
            println!(); 
        }

        if num == 4 {
            return
        }

        else {
            println!("Please enter a valid number."); 
            println!(); 
        }

    }
}
