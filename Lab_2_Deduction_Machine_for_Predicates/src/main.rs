use std::collections::HashMap;
use std::io::{self, Write};

fn main() {
    let mut premises = Vec::new();
    let mut deductions = Vec::new();

    loop {
        let mut input = String::new();
        println!("Enter a premise or deduction (or 'DONE' to finish): ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let input = input.trim();
        if input.eq_ignore_ascii_case("DONE") {
            break;
        }

        if input.starts_with("R ") {
            deductions.push(input[2..].trim().to_string());
        } else {
            premises.push(input.to_string());
        }
    }

    let evaluation_result = evaluate_premises(&premises, &deductions);
    match evaluation_result {
        EvaluationResult::Valid => println!("Deduction is valid."),
        EvaluationResult::Invalid => println!("Deduction is invalid."),
        EvaluationResult::InsufficientInfo => println!("Insufficient information."),
    }
}

fn evaluate_premises(premises: &[String], deductions: &[String]) -> EvaluationResult {
    let mut conditional_premises = HashMap::new(); // To store P → Q relationships

    // Parse premises for conditional statements
    for premise in premises {
        if let Some((p, q)) = parse_conditional_premise(premise) {
            conditional_premises.insert(p, q);
        }
    }

    // Implementing Law of Syllogism
    let mut derived_premises = conditional_premises.clone();
    for (p1, q1) in &conditional_premises {
        for (p2, q2) in &conditional_premises {
            if q1 == p2 {
                // We have found P → Q and Q → R, so add P → R to derived premises
                derived_premises.insert(p1.clone(), q2.clone());
            }
        }
    }

    // Evaluation with Modus Ponens, Modus Tollens, and Law of Syllogism
    for deduction in deductions {
        if premises.contains(deduction) || derived_premises.values().any(|q| q == deduction) {
            // Directly stated or derivable via Law of Syllogism, valid deduction
            return EvaluationResult::Valid;
        } else {
            for (p, q) in &derived_premises {
                // Modus Ponens: If P → Q and P, then Q.
                if premises.contains(p) && q == deduction {
                    return EvaluationResult::Valid;
                }
                // Modus Tollens: If P → Q and ¬Q, then ¬P.
                let negation_of_q = format!("¬{}", q); // Assuming negation is denoted by "¬"
                let negation_of_p = format!("¬{}", p); // The expected deduction is ¬P
                if premises.contains(&negation_of_q) && deduction == &negation_of_p {
                    return EvaluationResult::Valid;
                }
            }
        }
    }

    if deductions.is_empty() || derived_premises.is_empty() {
        EvaluationResult::InsufficientInfo
    } else {
        EvaluationResult::Invalid
    }
}

fn parse_conditional_premise(premise: &str) -> Option<(String, String)> {
    if premise.contains("->") { // Assuming "->" is used to denote implication
        let parts: Vec<&str> = premise.split("->").collect();
        if parts.len() == 2 {
            return Some((parts[0].trim().to_string(), parts[1].trim().to_string()));
        }
    }
    None
}

enum EvaluationResult {
    Valid,
    Invalid,
    InsufficientInfo,
}

