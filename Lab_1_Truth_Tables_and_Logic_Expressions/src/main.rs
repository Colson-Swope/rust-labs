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

        // count number of items in the vector 
        let mut itemCounter = values.len(); 
        println!("There are {} items in this vector", itemCounter); 

        // calculate the number of characters in the user input  
        for (index, value) in values.iter().enumerate() {
            number_amt = value.len() as i32; 
        }
        
        // process the characters 
        for string in &values {
            
            let mut stringPos = 0; 
            let mut reachEnd = false; 

            for character in string.chars() {                

                if character == '0' && stringPos == 0 {
                    print!("A"); 
                    stringPos += 1; 
                }
                else if character == '1' && stringPos == 0 {
                    print!("/A"); 
                    stringPos += 1; 
                }
                else if character == '0' && stringPos == 1 {
                    print!("B"); 
                    stringPos += 1; 
                }
                else if character == '1' && stringPos == 1 {
                    print!("/B");
                    stringPos += 1; 
                }
                else if character == '0' && stringPos == 2 {
                    print!("C"); 
                    stringPos += 1; 
                }
                else if character == '1' && stringPos == 2 {
                    print!("/C"); 
                    stringPos += 1; 
                }
                else if character == '0' && stringPos == 3 {
                    print!("D");
                    stringPos += 1;  
                }
                else if character == '1' && stringPos == 3 {
                    print!("/D");
                    stringPos += 1;     
                }
            }

            if itemCounter > 1 {
                itemCounter -= 1; 
                print!(" + "); 
    
            } 
        }   
        
        println!(); 
    }     
}
