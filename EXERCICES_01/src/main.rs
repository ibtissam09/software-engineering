fn main() {
    // println!("Hello, world!");
    exercise_variables_and_mutability();
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
