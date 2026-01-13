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




fn main() {
    println!("Hello, world!");
    // Test the longest_common_suffix function : Exercice 1
    // let str1 = "Runner";
    // let str2 = "Manner";
    // let suffix = longest_common_suffix("Runner", "Manner");
    // println!("Longest common suffix between '{}' and '{}' is '{}'", "Runner", "Manner", suffix);  

    // Test the Product struct and apply_discount method : Exercice 2
    let mut product = Product::new("Laptop", 1500.0, 10);
    println!("Original price of {}: ${}", product.name, product.price);
    product.apply_discount(0.1); // Apply a 10% discount
    println!("Price of {} after discount: ${}", product.name, product.price);
          
}
