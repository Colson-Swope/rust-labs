use std::io; 

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
            
            let mut stringPos = 0; 

            for character in string.chars() {
                println!("Character on deck: {}", character);                

                if character == '0' && stringPos == 0 {
                    println!("A"); 
                    stringPos += 1; 
                }
                else if character == '1' && stringPos == 0 {
                    println!("/A"); 
                    stringPos += 1; 
                }
                else if character == '0' && stringPos == 1 {
                    println!("B"); 
                    stringPos += 1; 
                }
                else if character == '1' && stringPos == 1 {
                    println!("/B");
                    stringPos += 1; 
                }
                else if character == '0' && stringPos == 2 {
                    println!("C"); 
                    stringPos += 1; 
                }
                else if character == '1' && stringPos == 2 {
                    println!("/C"); 
                    stringPos += 1; 
                }
                else if character == '0' && stringPos == 3 {
                    println!("D");
                    stringPos += 1; 
                }
                else if character == '1' && stringPos == 3 {
                    println!("/D");
                    stringPos += 1; 
                }
            }
        }   
    }     
}
