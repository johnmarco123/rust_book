use std::collections::HashMap;
// Given a list of integers, use a vector and return the median (when sorted, the value in the
// middle position) and mode (the value that occurs most often; a hash map will be helpful here) of
// the list.


fn bubble_sort(nums: &mut Vec<i32>) -> () {
    let len = nums.len();
    for i in 0..len {
        for j in 0..len - i - 1 {
            if nums[j] > nums[j + 1] {
                let temp = nums[j];
                nums[j] = nums[j + 1];
                nums[j + 1] = temp;
            }
        }
    };
}

fn median(nums: &mut Vec<i32>) -> f64 {
    let len = nums.len();
    if len == 0 {
        return 0.0
    }
    bubble_sort(nums);
    if len % 2 == 0 {
        let a = nums[len / 2] as f64;
        let b = nums[(len / 2) - 1] as f64;
        println!("a:{}, b{}", a, b);
        return (a + b) / 2.0 
         
    }
    nums[len / 2] as f64
}

fn mode(nums: &mut Vec<i32>) -> Vec<i32> {
    let mut map: HashMap<i32, i32> = HashMap::new();
    for n in nums {
        let count = map.entry(*n).or_insert(0);
        *count += 1;
    }

    let mut top_value = -1;
    let mut modes: Vec<i32> = Vec::new();

     for (key, value) in &map {
         if value == &top_value {
             modes.push(*key);
         } else if value > &top_value {
             modes.clear();
             modes.push(*key);
             top_value = *value;
         }
     }
     return modes
}
// Convert strings to pig latin. The first consonant of each word is moved to the end of
// the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have
// “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about
// UTF-8 encoding! 

// Using a hash map and vectors, create a text interface to allow a user to add
// employee names to a department in a company. For example, “Add Sally to Engineering” or “Add
// Amir to Sales.” Then let the user retrieve a list of all people in a department or all people in
// the company by department, sorted alphabetically.
fn main() {
    let mut list = vec![];
    println!("median of the list is {}", median(&mut list));
    println!("mode of the list is {:?}", mode(&mut list));
}
