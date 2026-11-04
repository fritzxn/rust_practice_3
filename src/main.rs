fn main() {
    //let mut v = Vec::new();
    let mut v = vec![1,2,3];
    v.push(3);
    v.push(6);

    let third: &i32 = &v[2];
    println!("The third elements is {third}");
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("no"),
    }
    for i in &v {
        println!("{i}");
    }
    for i in &mut v {
        *i += 50; // To change the value that the mutable reference 
        //refers to, we have to use the * dereference operator to get 
        //to the value in i before we can use the += operator
    }
    enum Spreadsheetcell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        Spreadsheetcell::Int(3),
        Spreadsheetcell::Float(10.12),
        Spreadsheetcell::Text(String::from("hello")),
    ];


//--------------------------------------------------
let data = "initial comment";
let mut s = String::new();
let s = data.to_string();
let s = "initial comment".to_string();
let s = String::from("initial comment");
let s1 = "woohoo";
let s2 = "bahah";
let new = s + &s1;// takes ownership of s
let new2 = format!("{s1}-{s2}");//only takes references of s1 and s2
//use caution when creating string slices with ranges, because doing 
//so can crash your program.
for c in "Зд".chars() {
    println!("{c}");
}
for b in "Зд".bytes() {
    println!("{b}");
}

use std::collections::HashMap;
let mut scores = HashMap::new();
scores.insert(String::from("blue"), 10);
scores.insert(String::from("yellow"), 50);
let team_name = String::from("blue");
let score = scores.get(&team_name).copied().unwrap_or(0);
for (key, value) in &scores {
    println!("{key}: {value}");
}












}