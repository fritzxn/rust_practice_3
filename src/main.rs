/* use std::array::from_fn;

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
use std::collections::HashMap;
let field_name = String::from("fav color");
let field_value = String::from("blue");
let mut map = HashMap::new();
map.insert(field_name, field_value);
// field name and value now invalid


let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Blue"), 25);
println!("{scores:?}");


let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.entry(String::from("yellow")).or_insert(50);
scores.entry(String::from("Blue")).or_insert(50);
println!("{scores:?}");
//The first call to entry will insert the key for the Yellow
// team with the value 50 because the Yellow team doesn’t have a 
//value already. The second call to entry will not change the hash 
//map, because the Blue team already has the value 10.


use std::collections::HashMap;
let text = "hello world wonderful world";
let mut map = HashMap::new();
for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}
println!("{map:?}");



} */




//---------------------------------------------------------------------------------------------------------------------------
//error handling


/* fn main () {
    let v = vec![1,2,3];
    v[99];
} */

/* use::std::fs::File;
use std::io::{self, ErrorKind}; */
/*fn main () {
    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!(
                        "problem creating the file: {e:?}",),
                }
                
            }
            _ => {
                    panic!("problem opening: {error:?}");
                }
        },
   };
} */

/* fn main() {
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("problem creating: {error:?}");
            })           
        } else {
            panic!(" problem openeing: {error:?}");
        }
    });
}
 */
/* use std::fs::File;
fn main() {
    let greeting_file = File::open("hello.txt").unwrap();
}

use std::fs::File;
fn main() {
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");
}
//choose expect rather than unwrap and give more context about 
//why the operation is expected to always succeed. That way, 
//if your assumptions are ever proven wrong, you have more 
//information to use in debugging. */

/* use::std::fs::File;
use std::io::{self, Read};
fn main() {

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut username = String::new();
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}
} */

/* use std::{fs::File, io::{self, Read}};

// the code above can be written as 
fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
} */
//If the value of the Result is an Ok, the value inside the Ok 
//will get returned from this expression, and the program will 
//continue. If the value is an Err, the Err will be returned from 
//the whole function as if we had used the return keyword so that 
//the error value gets propagated to the calling code.


//We could even shorten this code further by chaining method calls 
//immediately after the ?,

/* use std::{fs::File, io::{self, Read}};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
} */


//a way to make this even shorter using fs::read_to_string.

/* use std::{error::Error, fs, io};

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;
    Ok(())
} */

// can reread the value being evaluated again which is at the end of the chapter
//custom types for validation
//------------------------------------------------------------------

//GENERIC TYPES, TRAITS, AND LIFETIMES

fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let number_list = vec![23,234,65,444];
    let result = largest(&number_list);
    println!("largest is {result}");
    let char_list = vec!['e','r','w'];
    let result = largest(&char_list);
    println!("largest is {result}");
}//will produce an error

struct Point<T> {
    x: T,
    y: T,
}
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
fn main() {
    let p = Point {x: 5, y: 10};
    println!("p.x = {}", p.x());
}

fn main() {
    let integer = Point {x: 5, y: 7};
    let float = Point{x: 2.0, y: 4.3};
}
//both x and y have the same value type here

struct  Point<Y, U> {
    x: T,
    y: U,
}

fn main() {
    let both_integer = Point {x: 5, y: 10};
    let both_float = Point {x: 1.0, y: 4.0};
    let integer_and_float = Point {x: 5, y: 4.0};
}

//generics are very efficient cuz of monomorphization

pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
   }
}

let article = NewsArticle {
    headline: String::from(
        "Penguins win the Stanley Cup Championship!"
   ),
    location: String::from("Pittsburgh, PA, USA"),
    author: String::from("Iceburgh"),
    content: String::from(
        "The Pittsburgh Penguins once again are the best \
         hockey team in the NHL.",
   ),
};
println!("New article available! {}", article.summarize());

//traits is quite interesting

