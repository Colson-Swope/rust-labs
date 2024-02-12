use std::io; 
use num_bigint::BigInt;
use num_traits::{One, Zero};

use std::{thread, time}; 

fn factorial(num: u32) -> BigInt {
    if num == 1 || num == 0 {
        One::one()
    } else {
        let num_bigint = BigInt::from(num);
        num_bigint * factorial(num - 1)
    }
}

fn fibonacci(num: u32) -> BigInt {
    match num {
        0 => Zero::zero(),
        1 => One::one(),
        _ => fibonacci(num - 1) + fibonacci(num - 2),
    }
}

//https://iq.opengenus.org/different-ways-to-calculate-pi/
fn picalc(iterations: u32) -> f64 {

    // Initialize denominator
    let mut d = 1.0;
    // Initialize sum
    let mut sum = 0.0;

    for i in 0..2000000000 {
        if i % 2 == 0 {
            // Even index, add to sum
            sum += 4.0 / d;
        } else {
            // Odd index, subtract from sum
            sum -= 4.0 / d;
        }
        // Increment denominator by 2
        d += 2.0;
    }

    sum

}

fn modeselect(userFlag: u32) {

    if userFlag == 4 {

	std::process::exit(0);

    }

    println!(); 

    let mut numberInput = String::new(); 

    println!("Enter a numerical value for n: ");
    io::stdin().read_line(&mut numberInput).expect("Failed to read line"); 
    let num: u32 = numberInput.trim().parse().expect("Invalid input");

    println!(); 

    if userFlag == 1 {
        factorial(num);
        println!("Now printing the factorial: "); 
        println!("{}", factorial(num));
        thread::sleep(time::Duration::from_secs(2));
        println!();  
    }
 
    if userFlag == 2 {
        println!("Now printing the fibonacci: "); 
        print!("F{}: ", num);
        println!("{}", fibonacci(num));
	thread::sleep(time::Duration::from_secs(2));
	println!(); 
    }

    if userFlag == 3 {
        let pi = picalc(num);
        let number_Input_U32: usize = num as usize;

        println!("Now printing pi: ");
        println!("{:.1$}", pi, number_Input_U32);
	thread::sleep(time::Duration::from_secs(2));
	println!(); 
    }
}

fn main() {

    loop {
        println!("Welcome to the Factorial, Fibonacci, and Pi calculator");
        println!("1) Calculate factorial");
        println!("2) Calculate fibonacci");
        println!("3) Calculate pi"); 
	println!("4) Exit"); 
	println!();

        let mut userFlag = String::new();

        println!("Enter your choice: ");
        io::stdin().read_line(&mut userFlag).expect("Failed to read line");

        let userFlag: u32 = match userFlag.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
	
	if userFlag < 5 && userFlag > 0 {
	    modeselect(userFlag); 

	}
	else {
	    eprintln!("Input Error. Please enter a choice of 1-4"); 
	    println!(); 
	    main(); 
	}
    }
}
