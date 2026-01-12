fn main() {
    // println!("Hello, world!");
    exercise_variables_and_mutability();
    exercise_ownership_and_borrowing();
}

// Exercise 1: Variables and Mutability
// Goal: Practice declaring immutable and mutable variables, and understand their scope. 
// Instructions: 
// 1. Declare an immutable variable x with the value 10. 
// 2. Attempt to reassign x to 20 and observe the compiler error. 
// 3. Declare a mutable variable y with the value 15. 
// 4. Reassign y to 25 and print its value.

fn exercise_variables_and_mutability() {
    let x = 10;
    println!("The value of x is: {}", x);

    // Uncommenting the next line will cause a compiler error
    // x = 20; // This will not compile because x is immutable

    let mut y = 15;
    println!("The initial value of y is: {}", y);
    y = 25;
    println!("The new value of y is: {}", y);
}


// Exercise 2: Ownership and Borrowing
// Goal: Understand ownership transfer and borrowing.
// Instructions:
// 1. Create a String with the value "hello" and assign it to s1.
// 2. Transfer ownership of s1 to s2 and print s2.
// 3. Attempt to print s1 after transferring ownership and observe the compiler error.
// 4. Create a function print_string that takes an immutable reference to a String and
//    prints it.
// 5. Call print_string with s2.
// Hints:
//  Use String::from to create a String.
//  Use & to create an immutable reference.

fn print_string(s: &String) {
    println!("The string is: {}", s);
}

fn exercise_ownership_and_borrowing() {
    let s1 = String::from("hello");
    let s2 = s1; // Ownership of s1 is transferred to s2
    println!("s2: {}", s2);

    // Uncommenting the next line will cause a compiler error
    // println!("s1: {}", s1); // This will not compile because s1 is no longer valid

    print_string(&s2);
}