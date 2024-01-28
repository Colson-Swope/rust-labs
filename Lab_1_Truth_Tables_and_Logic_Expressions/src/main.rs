use std::io; 

fn main() {

    loop {
        let mut number_amt: i32 = 0; 

        // prompt user instructions 
        // get user input numbers
        let mut line = String::new(); 
        println!("Enter a string of numbers, either separated by commas, whitespace between numbers, or both. Type 'Q' to quit: ");      

        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line"); 

        // split commas, collect values into a vector 
       
        let values: Vec<String> = line
            .split(|c| c == ',' || c == ' ')
            .filter(|s| !s.trim().is_empty())
            .map(|s| s.trim().to_string())
            .collect(); 

        // count number of items in the vector 
        let mut item_counter = values.len();  

        // calculate the number of characters in the user input  
        for value in &values {
            number_amt = value.len() as i32; 
        }
        
        // process the characters 
        for string in &values {
            let mut string_pos = 0;  
            let mut error_exit = false; 

            for character in string.chars() {          

                if character != '0' && character != '1' {
                    println!("Error: Only 1's and 0's can be used as valid input");
                    error_exit = true;
                    break; 
                }

                if number_amt > 4 { 
                    println!("Error: Please only enter up to 4 numbers per expression"); 
                    error_exit = true;
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
                    println!("D");
                    string_pos += 1;  
                }
                else if character == '1' && string_pos == 3 {
                    println!("/D");
                    string_pos += 1;     
                }
                else {
                    break; 
                }
            }

            if item_counter > 1 && error_exit == false {
                item_counter -= 1; 
                print!(" + ");  
            } 
        }   
        
        println!(); 
    }     
}
