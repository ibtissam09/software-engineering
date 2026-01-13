fn main() {
    variables_and_mutability();
    ownership_example();
    borrowing_example();
    if_else_example(7);
    loops_example();
    loop_control();
    string_example();
}

// Variables and Mutability
fn variables_and_mutability() {
    let x = 5;
    println!("The value of x is: {}", x);
    //x = 6;
    println!("The value of x is: {}", x);
}

fn ownership_example() {
    //Int is a primitive type and implements the Copy trait (stack data)
    let i1 = 10;
    let i2 = i1; // the value of i1 is copiend into i2
    println!("i1: {}, i2: {}", i1, i2);
    //String is a non-primitive type and does not implement the Copy trait (heap data)
    let s1 = String::from("hello");
    let s2 = s1;
    //Error! s1 is no longer valid here
    //println!("s1: {}, s2: {}", s1, s2);
}

fn borrowing_example() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // pass a reference to s1
    println!("The length of '{}' is {}.", s1, len);

    let mut s2 = String::from("hello");
    change_string(&mut s2); // pass a mutable reference to s2
    println!("s2 after change: {}", s2);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change_string(s: &mut String) {
    s.push_str(", world");
}

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

// 5. Arithmetic Operators
fn arithmetic_operators() {
    let sum = 5 + 3;
    let difference = 10 - 4;
    let product = 4 * 5;
    let quotient = 16 / 4;
    let remainder = 17 % 4;

    println!(
        "Sum: {}, Difference: {}, Product: {}, Quotient: {}, Remainder: {}",
        sum, difference, product, quotient, remainder
    );
}

// 6. Functions and Procedures
fn add(a: i32, b: i32) -> i32 {
    a + b // Implicit return.
}

fn greet(name: &str) {
    println!("Hello, {}!", name); // Procedure: no return value.
}

fn if_else_example(x: i32) {
    if x < 5 {
        println!("x is less than 5");
    } else if x == 5 {
        println!("x is equal to 5");
    } else {
        println!("x is greater than 5");
    }
}

fn boolean_operators() {
    let age = 25;

    let is_member = true;
    let is_promotion_active = false;

    if (age < 18 || age >= 65) && is_member {
        println!("Eligible for child or Senior discount");
    } else if (age >= 18 && age < 65) && is_promotion_active {
        println!("Eligible for standard promotion discount");
    } else if is_member {
        println!("Eligible for member discount");
    } else {
        println!("No discounts available");
    }
}

fn match_example(number: i32) {
    match number {
        1 => println!("One"),
        2 | 3 => println!("Two or Three"),
        4..=10 => println!("Between Four and Ten"),
        _ => println!("Greater than Ten or Less than One"),
    }
}

fn loops_example() {
    //infinite loop
    let mut counter = 0;
    loop {
        counter += 1;
        if counter > 5 {
            break; //Exit the loop
        }
    }

    //While loop
    let mut x = 0;
    while x < 5 {
        println!("x: {}", x);
        x += 1;
    }

    //For loop
    for i in 0..5 {
        println!("i: {}", i);
    }

    //Iterate over an array
    let myArray = [0, 1, 2, 3, 4];
    for element in myArray.iter() {
        println!("element: {}", element);
    }
}

fn loop_control() {
    for i in 0..5 {
        if i % 2 == 0 {
            continue; //skip even numbers
        }
        println!("odd number: {}", i);
    }
    let mut x = 0;
    /*'outer: loop {
        loop {
            println!("In the inner loop");
            if x > 5 {
                break 'outer; //break the outer loop
            }
            x += 1;
        }
    }

    let mut count: i32 = loop {
        break 42;
    };*/
}

fn string_example() {
    let mut s = String::from("Hello");
    s.push_str(", world!");
    println!("{}", &s);

    for c in s.chars() {
        println!("{}", c);
    }

    let slice = &s[0..5];
    println!("Slice: {}", slice);
    s = String::from("Goodbye");
    print!("Modified String: {}", s);
}
