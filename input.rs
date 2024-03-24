use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // Read the input from the file
    if let Ok(lines) = read_lines("input.txt") {
        let mut max_calories = 0;
        let mut elf_calories = Vec::new(); // Vector to store total calories for each Elf

        // Variable to store the sum of Calories carried by the current Elf
        let mut current_calories = 0;

        for line in lines.flatten() {
            if line.trim().is_empty() {
                // Blank line encountered, reset the sum of Calories
                max_calories = max_calories.max(current_calories);
                elf_calories.push(current_calories); // Store the current Elf's total calories
                current_calories = 0;
            } else {
                // Parse the line into an integer representing Calories
                if let Ok(calories) = line.trim().parse::<i32>() {
                    // Add the Calories to the sum of Calories carried by the current Elf
                    current_calories += calories;
                }
            }
        }

        // Check if the sum of Calories carried by the last Elf exceeds the current maximum
        max_calories = max_calories.max(current_calories);
        elf_calories.push(current_calories); // Store the last Elf's total calories

        // Print the total Calories carried by the Elf with the most Calories
        println!("Total Calories carried by the Elf with the most Calories: {}", max_calories);

        // Print the array of total Calories carried by each Elf
        println!("Total Calories carried by each Elf: {:?}", elf_calories);

        // Perform an operation on the array of total Calories (without revealing what it does)
        let result = secret_operation(&elf_calories);
        println!("Secret operation result: {}", result);
    }
}

// Function to read lines from a file
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// Secret operation on the array of total Calories
fn secret_operation(elf_calories: &[i32]) -> i32 {
    let mut result = 0;
    for &calories in elf_calories {
        result += calories % 10; // Example secret operation: Sum of the last digit of each calorie count
    }
    result
}

