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


fn main() {
    println!("Hello, world!");
    // Test the longest_common_suffix function : Exercice 1
    // let str1 = "Runner";
    // let str2 = "Manner";
    // let suffix = longest_common_suffix("Runner", "Manner");
    // println!("Longest common suffix between '{}' and '{}' is '{}'", "Runner", "Manner", suffix);        
}
