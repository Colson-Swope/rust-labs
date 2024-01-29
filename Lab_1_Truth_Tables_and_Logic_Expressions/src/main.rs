// I/O Functionality 
use std::io; 

fn main() {

    loop {
        let mut number_amt: i32 = 0; // # of chars in user input  
        let mut error_flag = false; // flag used to check for errors 

        // prompt user instructions 
        // get user input numbers
        let mut line = String::new(); 
        println!("Enter a string of numbers, either separated by commas, whitespace between numbers, or both. Type 'Q' to quit: ");      

        // read users line of text 
        // throw error if fail to read line 
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line"); 

        // split commas, collect values into a vector 
        let values: Vec<String> = line
            // if user enters a comma or space, split into different inputs
            .split(|c| c == ',' || c == ' ')
            // remove whitespace 
            .filter(|s| !s.trim().is_empty())
            // remove whitespace, convert to string 
            .map(|s| s.trim().to_string())
            // convert strings to a vector 
            .collect(); 

        // count number of items in the vector 
        let mut item_counter = values.len();  

        // calculate the number of characters in the user input  
        for value in &values {
            number_amt = value.len() as i32; 
        }
        
        // process the characters 
        for string in &values {
            // monitor position of vector iterator 
            let mut string_pos = 0;  

            // while there are characters, decide what to do with them 
            for character in string.chars() {          

		// quit program if user enters "q" or "Q" 
                if character == 'q' || character == 'Q' {
                    error_flag = true; 
                    return;
                }

		// error if user enters foreign character 
                if character != '0' && character != '1' { 
                    eprintln!("Error: Only 1's and 0's are accepted as valid input");
                    error_flag = true; 
                    break;
                }

		// error if user enters more than 4 characters at a time 
                if number_amt > 4 {  
                    eprintln!("Error: Only four characters per input row");   
                    error_flag = true; 
                    break; 
                }
 
                if character == '0' && string_pos == 0 {
                    print!("A"); 
                    string_pos += 1; 
                }

                else if character == '1' && string_pos == 0 {
                    print!("/A"); 
                    string_pos += 1; 
                }
                else if character == '0' && string_pos == 1 {
                    print!("B"); 
                    string_pos += 1; 
                }
                else if character == '1' && string_pos == 1 {
                    print!("/B");
                    string_pos += 1; 
                }
                else if character == '0' && string_pos == 2 {
                    print!("C"); 
                    string_pos += 1; 
                }
                else if character == '1' && string_pos == 2 {
                    print!("/C"); 
                    string_pos += 1; 
                }
                else if character == '0' && string_pos == 3 {
                    print!("D");
                    string_pos += 1;  
                }
                else if character == '1' && string_pos == 3 {
                    print!("/D");
                    string_pos += 1;     
                }
                else {
                    break; 
                }
            }

	    // if one of our conditions sets this flag to true, stop processing input  
            if error_flag == true {
                break; 
            }

            // do not print '+' character if it is the last input 
            if item_counter > 1 {
                item_counter -= 1; 
                print!(" + ");  
            } 
        }   
        
        // if one of our conditions sets this flag to true, restart the loop 
        if error_flag == true {
            continue; 
        }

        println!(); 
    }     
}
