use std::collections::HashMap;

enum SpreadsheetCell
{
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() 
{
    // Vectors (variable sized arrays in the heap)
    let mut v: Vec<i32> = Vec::new();

    // using the macro, infers types
    let v_2 = vec![1, 2, 3, 3, 3, 4, 5];

    v.push(5);
    v.push(6);
    v.push(7);


    let third: &i32 = &v_2[2];
    println!("The third element is {}", third);

    let third: Option<&i32> = v.get(2);
    match third
    {
        Some(third) =>  println!("The third element is {}.", third),
        None =>         println!("There is no third element.")
    }

    for n_ref in &v
    {
        // the extracted loop variable is a reference to protect ownership, but means we have to de-reference
        let n_plus_one: i32 = *n_ref+1;
        println!("{}",n_plus_one);
    }

    let _row = vec!
    [
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("Blue")),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Int(45),
    ];

    println!("The median is {}.", median(&v_2));
    println!("The mode is {}.", mode(&v_2));

    let test_word = String::from("avocado");
    println!("{}",pig_latin(&test_word));
    println!("{}",test_word);

}

fn median(list: &Vec<i32>) -> i32
{
    // get a clone I can sort without modifying the input
    let mut sorted_list = list.clone();
    sorted_list.sort();

    let list_length = sorted_list.len();

    let list_idx = list_length/2;

    return sorted_list[list_idx];
}

fn mode(list: &Vec<i32>) -> i32
{
    let mut tracker: HashMap<i32, usize> = HashMap::new();

    // Count all the numbers to later find the mode
    for number in list
    {
        let count = tracker.entry(*number).or_insert(0);
        *count += 1;
    }

    // Find the max value
    let mut current_max: (i32, usize) = (0, 0);

    for (key, value) in &tracker
    {
        if *value > current_max.1 
        {
            current_max.0 = *key;
            current_max.1 = *value;
        }
    }

    return current_max.0;
}

fn pig_latin(word: &String) -> String
{
    let vowels = ['a','e','i','o','u'];

    let first_char = &word[0..1];
    let lower_case_first_char = first_char.to_lowercase();
    let headless = &word[1..];

    let s: String;
    if vowels.iter().any(|&v| first_char.starts_with(v))
    {
        s = format!("{}hay",headless);
    }
    else
    {
        s =  format!("{}{}ay",headless,lower_case_first_char);
    }

    return s;
}