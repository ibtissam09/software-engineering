mod financial_portfolio_analysis;

use financial_portfolio_analysis::*;

// Exercise 1: Lifetime
// Objective: Practice using lifetime annotations to ensure references remain valid. 
// Task: Write a function longest_common_suffix that takes two string slices and returns 
// the longest common suffix. If there is no common suffix, return an empty string slice. 
// Requirements:
// • Use lifetime annotations.
// • Handle edge cases (e.g., empty strings, no common suffix). Method/Function 
// Information:
// • Use str::chars() to iterate over characters.
// • Use str::rev() to reverse the string for easier suffix comparison.

fn longest_common_suffix<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    let rev_s1: Vec<char> = s1.chars().rev().collect();
    let rev_s2: Vec<char> = s2.chars().rev().collect();
    
    let mut common_suffix = String::new();
    
    for (c1, c2) in rev_s1.iter().zip(rev_s2.iter()) {
        if c1 == c2 {
            common_suffix.push(*c1);
        } else {
            break;
        }
    }
    
    let suffix: String = common_suffix.chars().rev().collect();
    let suffix_len = suffix.len();
    
    if suffix_len == 0 {
        ""
    } else {
        &s1[s1.len() - suffix_len..]
    }
}

/// Exercise 2: Structs
/// Objective: Implement a struct and associated methods. 
/// Task: Define a struct Product with fields name, price, and stock. Implement a method 
/// apply_discount that reduces the price by a given percentage. 
/// Requirements:
/// • Use an associated function to create a new Product instance.
/// • Implement the apply_discount method. Method/Function Information:
/// • Use f64 for the price to handle decimal values.
/// • Use self.price *= (1.0 - discount) to apply the discount.

struct Product {
    name: String,
    price: f64,
    stock: u32,
}

impl Product {
    fn new(name: &str, price: f64, stock: u32) -> Self {
        Product {
            name: name.to_string(),
            price,
            stock,
        }
    }

    fn apply_discount(&mut self, discount: f64) {
        if discount >= 0.0 && discount <= 1.0 {
            self.price *= 1.0 - discount;
        }
    }
}


// Exercise 3: Generics
// Objective: Use generic types to create a flexible function. 
// Task: Write a generic function swap that swaps the values of two variables of the same 
// type. 
// Requirements:
// • Use generic type parameters. Method/Function Information:
// • Use a tuple to return the swapped values.

fn swap<T>(a: T, b: T) -> (T, T) {
    (b, a)
}

// Exercise 4: Option
// Objective: Practice handling optional values. 
// Task: Write a function find_index that takes a vector of integers and a target value. 
// Return the index of the target value as Some(index) if found, otherwise return None. 
// Requirements:
// • Use Option<usize> as the return type. Method/Function Information:
// • Use iter().enumerate() to iterate over the vector with indices.


fn find_index(vec: &Vec<i32>, target: i32) -> Option<usize> {
    for (index, &value) in vec.iter().enumerate() {
        if value == target {
            return Some(index);
        }
    }
    None
}

/// Exercise 5: Result
/// Objective: Practice error handling with Result. 
/// Task: Write a function safe_sqrt that takes a f64 and returns the square root as Ok(f64) if 
/// the input is non-negative, otherwise return an Err(String). 
/// Requirements:
/// • Use Result<f64, String> as the return type. Method/Function Information:
/// • Use f64::sqrt() to compute the square root.
/// • Return Err with a message if the input is negative

fn safe_sqrt(x: f64) -> Result<f64, String> {
    if x < 0.0 {
        Err("Input must be a non-negative number".to_string())
    } else {
        Ok(x.sqrt())
    }
}


/// Exercise: Financial Portfolio Analysis
/// Objective: Use iterators and data structures to analyze a financial portfolio. 
/// Calculate the total value, diversification score, and identify high-risk assets.
/// Task: You are given a dataset of financial assets in a portfolio. Each asset 
/// includes a ticker symbol, quantity, price per unit, and risk level (low, medium, 
/// high). Your task is to:
/// 1. Calculate the total value of the portfolio.
/// 2. Calculate a diversification score (number of unique asset classes).
/// 3. Identify assets with a "high" risk level.
/// 4. Filter out assets with a value below a specified threshold (e.g., 1000.0).
/// Requirements:
/// • Use HashMap to aggregate the total value by asset class.
/// • Use iterators (map, filter, fold) to process the data.
/// • Choose between HashMap or BTreeMap for aggregation.
/// Output :
/// Total portfolio value: 12700.00
/// Diversification score (unique assets): 4
/// High-risk assets: [Asset { ticker: "TSLA", quantity: 5, price: 700.0, risk: High }, Asset {
/// ticker: "NVDA", quantity: 4, price: 200.0, risk: High }]
/// Assets with value >= 1000.0: [Asset { ticker: "TSLA", quantity: 5, price: 700.0, risk: High },
/// Asset { ticker: "AMZN", quantity: 2, price: 3000.0, risk: Medium }, Asset { ticker: "GOOGL",
/// quantity: 3, price: 2500.0, risk: Medium }





fn main() {
    println!("Hello, world!");
    // Test the longest_common_suffix function : Exercice 1
    // let str1 = "Runner";
    // let str2 = "Manner";
    // let suffix = longest_common_suffix("Runner", "Manner");
    // println!("Longest common suffix between '{}' and '{}' is '{}'", "Runner", "Manner", suffix);  

    // Test the Product struct and apply_discount method : Exercice 2
    /*
    let mut product = Product::new("Laptop", 1500.0, 10);
    println!("Original price of {}: ${}", product.name, product.price);
    product.apply_discount(0.1); // Apply a 10% discount
    println!("Price of {} after discount: ${}", product.name, product.price);
    */
    // Test the swap function : Exercice 3
    /*
    let x = 10;
    let y = 20;

    let (new_x, new_y) = swap(x, y);
    println!("new_x = {}, new_y = {}", new_x, new_y);

    let a = "hello";
    let b = "world";

    let (new_a, new_b) = swap(a, b);
    println!("new_a = {}, new_b = {}", new_a, new_b);
    */
    // Test the find_index function : Exercice 4
    
    // let numbers = vec![10, 20, 30, 40, 50, 60];
    // let target = 60;
    // match find_index(&numbers, target) {
    //     Some(index) => println!("Found {} at index {}", target, index),
    //     None => println!("{} not found in the vector", target),
    // }
    // Test the safe_sqrt function : Exercice 5
    
    //let number = 9.0;
    // match safe_sqrt(number) {
    //     Ok(result) => println!("The square root of {} is {}", number, result),
    //     Err(e) => println!("Error: {}", e), 
    // }
    // Test the financial portfolio analysis functions : Exercice 6
    let assets = portfolio();
    let total = total_value(&assets);
    let diversification = diversification_by_risk(&assets);
    let diversification_score = diversification_score(&diversification);
    let high_risk = high_risk_assets(&assets);
    let filtered_assets = assets_above_threshold(&assets, 1000.0);
    println!("Total portfolio value: {:.2}", total);
    println!("Diversification score (unique assets): {}", diversification_score);
    println!("High-risk assets: {:?}", high_risk);
    println!("Assets with value >= 1000.0: {:?}", filtered_assets);
}   
