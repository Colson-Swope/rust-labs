use std::io; 

// 10101
// A/BC/D 

// 00001
// /A/B/C/D

// 1111
// ABC

fn main() {

    loop {

        // prompt user instructions 
        // get user input numbers
        let mut line = String::new(); 
        println!("Enter a string of numbers, either separated by commas, whitespace between numbers, or both. Type 'Q' to quit: ");
         
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line"); 
            
        // split commas, collect values into a vector 
        let values: Vec<String> = line.trim().split(',').map(|s| s.trim().to_string()).collect(); 
        
        // Debug 
        for (index, value) in values.iter().enumerate() {
            let number_amt = value.len(); 
            println!("Value {} is: {}", index + 1, value);
            print!("Value for value {} is {}\n", index, number_amt);
        }

        // if value of input is 1, output only A, if value of input is 2, input A and B, etc. 
    }     
}
