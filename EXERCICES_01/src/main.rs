fn main() {
    // println!("Hello, world!");
    exercise_variables_and_mutability();
    exercise_ownership_and_borrowing();
    process_sensor_data(vec![
        ("sensor_1", vec![22.5, 23.0, 22.8, -60.0, 23.3]),
        ("sensor_2", vec![18.0, 19.5, 18.7, 20.0, 19.2]),
        ("sensor_3", vec![25.0, 24.8, 25.2, 25.1, 24.9]),
    ]);
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

// Exercise 3: Data Processing Pipeline
// Goal: Implement a data processing pipeline that filters, transforms, and aggregates data
// using Rust’s core concepts.
// Scenario:
// You are given a dataset of temperature readings (in Celsius) from multiple sensors over a
// 24-hour period. Your task is to:
// 1. Filter out invalid readings (values below -50°C or above 60°C).
// 2. Convert valid readings from Celsius to Fahrenheit.
// 3. Calculate the average temperature for each sensor.
// 4. Identify the sensor with the highest average temperature.
// Data:
// Use the following dataset (simulated as a vector of tuples):
// let sensor_data: Vec<(&str, Vec<f64>)> = vec![
// ("sensor_1", vec![22.5, 23.0, 22.8, -60.0, 23.3]),
// ("sensor_2", vec![18.0, 19.5, 18.7, 20.0, 19.2]),
// ("sensor_3", vec![25.0, 24.8, 25.2, 25.1, 24.9]),
// ];
// Instructions:
// 1. Define a function filter_invalid_readings that takes a vector of f64 and returns a
// new vector with only valid readings.
// 2. Define a function ahrenh_to_fahrenheit that converts a Celsius value to
// Fahrenheit.
// 3. Define a function calculate_average that calculates the average of a vector of f64.
// 4. Define a function process_sensor_data that:
// o Filters invalid readings for each sensor.
// o Converts valid readings to Fahrenheit.
// 1. F = (C × 9/5) + 32
// o Calculates the average temperature for each sensor.
// o Returns the sensor name with the highest average temperature.
// 5. Print the results for each sensor and the sensor with the highest average
// temperature.
// Hints:
//  Use filter and map to process the data.
//  Use a match expression or if statements to filter invalid readings.
//  Use a loop or iterator to calculate the average for each sensor.
//  Use a struct or tuple to store intermediate results (e.g., sensor name and average
// temperature).
// Expected Outcome:
//  Students should print the average temperature for each sensor in Fahrenheit.
//  Students should identify and print the sensor with the highest average
// temperature.

fn filter_invalid_readings(readings: &Vec<f64>) -> Vec<f64> {
    readings
        .iter()
        .cloned()
        .filter(|&temp| temp >= -50.0 && temp <= 60.0)
        .collect()
}
fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    (celsius * 9.0 / 5.0) + 32.0
}

fn calculate_average(readings: &Vec<f64>) -> f64 {
    let sum: f64 = readings.iter().sum();
    sum / readings.len() as f64
}

fn process_sensor_data(sensor_data: Vec<(&str, Vec<f64>)>) {
    let mut highest_avg_temp = std::f64::MIN;
    let mut sensor_with_highest_avg = "";

    for (sensor_name, readings) in sensor_data {
        let valid_readings = filter_invalid_readings(&readings);
        let fahrenheit_readings: Vec<f64> = valid_readings
            .iter()
            .map(|&temp| celsius_to_fahrenheit(temp))
            .collect();
        let average_temp = calculate_average(&fahrenheit_readings);

        println!(
            "Sensor: {}, Average Temperature (F): {:.2}",
            sensor_name, average_temp
        );

        if average_temp > highest_avg_temp {
            highest_avg_temp = average_temp;
            sensor_with_highest_avg = sensor_name;
        }
    }

    println!(
        "Sensor with highest average temperature: {} ({:.2} F)",
        sensor_with_highest_avg, highest_avg_temp
    );
}