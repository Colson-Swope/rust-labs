use std::io;

// determines if our set relation is reflexive 
fn is_reflexive(relation: &[(String, String)], set: &[String]) -> bool {
    for element in set {
        if !relation.iter().any(|&(ref a, ref b)| a == element && b == element) {
            return false;
        }
    }
    true
}

// determines if our set relation is symmetric 
fn is_symmetric(relation: &[(String, String)]) -> bool {
    for &(ref a, ref b) in relation {
        if !relation.iter().any(|&(ref x, ref y)| x == b && y == a) {
            return false;
        }
    }
    true
}

// determines if our set relation is transitive 
fn is_transitive(relation: &[(String, String)]) -> bool {
    for &(ref a, ref b) in relation {
        for &(ref x, ref y) in relation {
            if x == b && !relation.iter().any(|&(ref m, ref n)| m == a && n == y) {
                return false;
            }
        }
    }
    true
}

// determines if our set relation is antisymmetric 
fn is_antisymmetric(relation: &[(String, String)]) -> bool {
    for &(ref a, ref b) in relation {
        if a != b && relation.iter().any(|&(ref x, ref y)| x == b && y == a) {
            return false;
        }
    }
    true
}

// determines if our set relations are equivalent 
fn is_equivalence(relation: &[(String, String)], set: &[String]) -> bool {
    is_reflexive(relation, set) && is_symmetric(relation) && is_transitive(relation)
}

fn main() {
    let mut input_one = String::new();
    println!("Enter a set: ");
    io::stdin().read_line(&mut input_one).expect("Error: Failed to read set input");

    let set_input: Vec<String> = input_one
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();

    let mut input_two = String::new();
    println!("Enter pairs (comma-separated, e.g., (a,b) (c,d)): ");
    io::stdin().read_line(&mut input_two).expect("Error: Failed to read pair input");

    let mut pair_input: Vec<(String, String)> = Vec::new();
    for pair in input_two.split_whitespace() {
        if pair.starts_with('(') && pair.ends_with(')') && pair.contains(',') {
            let pair_str = &pair[1..pair.len() - 1];
            let elements: Vec<&str> = pair_str.split(',').collect();
            if elements.len() == 2 {
                pair_input.push((elements[0].trim().to_string(), elements[1].trim().to_string()));
            } else {
                println!("Invalid pair: {}", pair_str);
                return
            }
        } else {
            println!("Invalid pair: {}", pair);
            return 
        }
    }

    println!("Is reflexive: {}", is_reflexive(&pair_input, &set_input));
    println!("Is symmetric: {}", is_symmetric(&pair_input));
    println!("Is transitive: {}", is_transitive(&pair_input));
    println!("Is antisymmetric: {}", is_antisymmetric(&pair_input));
    println!("Is equivalence: {}", is_equivalence(&pair_input, &set_input));
}