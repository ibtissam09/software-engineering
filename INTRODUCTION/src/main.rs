fn main() {
    println!("Hello, world!");
    //loops_example();
    //loop_control();
    //string_example();
    data_types();
}

// Data Types and Variables
fn data_types() {
    //Scalar types
    let a: u8 = 255;
    let b: i8 = -128;
    let c: f64 = 7.3;
    let d: bool = true;
    let e: char = 'Z';

    //Compound types
    let tuple: (i32, f32, u8) = (42, 7.3, 1);
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    let vec: Vec<i32> = vec![1, 2, 3];
    let string = String::from("hello");
    let slice: &str = "world";

    println!(
        "Tuple: {:?}, Array: {:?}, Vector: {:?}, String: {}, Slice: {}",
        tuple, array, vec, string, slice
    );
}


fn loops_example() {
    let mut count: i32 = 0;
    loop {
        count += 1;
        if count >= 5 {
            break;
        }
    }
    // While loop
    let mut number: i32 = 0;
    while number < 5 {
        println!("Number: {}", number);
        number += 1;    
    }
    // For loop
    for i in 0..5 {
        println!("For loop iteration: {}", i);
    }
    // Iterating over an array
    let arr = [10, 20, 30, 40, 50];
    for element in arr.iter() {
        println!("Array element: {}", element);
    }   
}

fn loop_control(){
    for i in 0..10 {
        if i % 2 == 0 {
            continue; // Skip even numbers
        }
        if i > 7 {
            break; // Exit loop if i is greater than 7
        }
        println!("Odd number less than or equal to 7: {}", i);
    }  
    // Outer and inner loop with labels
    
}

fn string_example() {
    let sample_str = String::from("Hello, Rust!");
    for c in sample_str.chars() {
        println!("{}", c);
    }
}


// Pointers and References
