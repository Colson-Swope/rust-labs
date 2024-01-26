use std::io; 

// 10101
// A/BC/D 

// 00001
// /A/B/C/D

// 1111
// ABC

fn main() {

    loop {
        let mut number_amt: i32 = 0; 

        let invertOne = false; 
        let invertTwo = false; 

        // prompt user instructions 
        // get user input numbers
        let mut line = String::new(); 
        println!("Enter a string of numbers, either separated by commas, whitespace between numbers, or both. Type 'Q' to quit: ");      

        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line"); 
            
        // split commas, collect values into a vector 
        let values: Vec<String> = line.trim().split(',').map(|s| s.trim().to_string()).collect();  

        // calculate the number of characters in the user input  
        for (index, value) in values.iter().enumerate() {
            number_amt = value.len() as i32; 
        }
        
        // process the characters 
        for string in &values {
            for character in string.chars() {
                println!("Character on deck: {}", character); 
            }
        }   
    }     
}
