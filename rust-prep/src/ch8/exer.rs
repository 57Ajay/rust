use std::collections::HashMap;
use std::io::{self};
//Problem 1
fn sort_arr(arr: &mut Vec<i32>) -> Vec<i32> {
    let len = arr.len();
    for _ in 0..len {
        for i in 0..len - 1 {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
            }
        }
    }
    arr.to_vec()
}

fn find_median(arr: &Vec<i32>) -> f32 {
    let len = arr.len();
    if len % 2 == 0 {
        (arr[len / 2 - 1] + arr[len / 2]) as f32 / 2.0
    } else {
        arr[len / 2] as f32
    }
}

fn find_mode(arr: &Vec<i32>) -> i32 {
    let mut occurrences = HashMap::new();

    for &value in arr.iter() {
        let count = occurrences.entry(value).or_insert(0);
        *count += 1;
    }

    let mode = occurrences
        .iter()
        .max_by_key(|&(_, count)| count)
        .map(|(&value, _)| value)
        .unwrap_or(0);

    mode
}

fn vec_medium_mode() {
    let mut arr = vec![21, 14, 65, 76, 98, 11, 76, 14, 14];
    sort_arr(&mut arr); // we could use rust builtin method named sort() like arr.sort();
                        //arr.sort();

    let median = find_median(&arr);
    let mode = find_mode(&arr);

    println!("Sorted array: {:?}", arr);
    println!("Median: {}", median);
    println!("Mode: {}", mode);
}

//Problem 2
//
fn is_vowel(c: char) -> bool {
    matches!(c, 'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U')
}

fn to_pig_latin(word: &str) -> String {
    let mut chars = word.chars();
    let first_char = chars.next().unwrap();
    println!("chars: {chars:#?}, first_char: {first_char}");
    if is_vowel(first_char) {
        format!("{}-hay", word)
    } else {
        let rest_of_word: String = chars.collect();
        println!("Rest of the word: {rest_of_word}");
        format!("{}-{}ay", rest_of_word, first_char)
    }
}

fn convert_sentence_to_pig_latin(sentence: &str) -> String {
    sentence
        .split_whitespace()
        .map(|word| to_pig_latin(word))
        .collect::<Vec<_>>()
        .join(" ")
}

//Problem 3;
//Solved with AI;

// Function to handle adding an employee to a department
fn handle_add_employee(input: &str, company: &mut HashMap<String, Vec<String>>) {
    // Parse the input string, assuming the format is: "Add <Employee> to <Department>"
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.len() < 4 || parts[1].is_empty() || parts[3].is_empty() || parts[2] != "to" {
        println!("Invalid format. Use 'Add <Employee> to <Department>'");
        return;
    }

    let employee = parts[1].to_string(); // Employee's name
    let department = parts[3..].join(" ").to_uppercase(); // Department name (in case it has spaces)

    // Insert the employee into the department in the HashMap
    company
        .entry(department.clone())
        .or_insert_with(Vec::new)
        .push(employee.clone());

    println!("Added {} to {}", employee, department);
}

// Function to handle listing employees in a specific department
fn handle_list_employees_in_department(department: &str, company: &HashMap<String, Vec<String>>) {
    if let Some(employees) = company.get(department) {
        let mut sorted_employees = employees.clone();
        sorted_employees.sort(); // Sort employees alphabetically
        println!("Employees in {} department:", department);
        for employee in sorted_employees {
            println!(">> {}", employee);
        }
    } else {
        println!("No such department found.");
    }
}

// Function to list all employees by department, sorted alphabetically
fn handle_list_all_employees(company: &HashMap<String, Vec<String>>) {
    let mut departments: Vec<&String> = company.keys().collect();
    departments.sort(); // Sort departments alphabetically

    for department in departments {
        let mut employees = company.get(department).unwrap().clone();
        employees.sort(); // Sort employees within each department
        println!("{} department:", department);
        for employee in employees {
            println!(">>   {}", employee);
        }
    }
}

pub fn main() {
    vec_medium_mode();

    let sentence = "first apple banana orange";
    let pig_latin_sentence = convert_sentence_to_pig_latin(sentence);
    println!("{}", pig_latin_sentence);

    let mut company: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        println!("\nChoose an option:");
        println!("1. Add employee to department (e.g., 'Add Sally to Engineering')");
        println!("2. Get list of employees in a department (e.g., 'ENGINEERING', Make sure all the chars are Uppercase.)");
        println!("3. Get list of all employees by department (enter 3)");
        println!("4. Exit");

        // Get input from the user
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim();

        if input == "4" {
            println!("Exiting the application.");
            break;
        } else if input.starts_with("Add ") {
            // Example: "Add Sally to Engineering"
            handle_add_employee(input, &mut company);
        } else if company.contains_key(input) {
            // If the input is a department name, display employees in the department
            handle_list_employees_in_department(input, &company);
        } else if input == "3" {
            // List all employees in the company by department
            handle_list_all_employees(&company);
        } else {
            println!("Invalid input, please try again.");
        }
    }
}
