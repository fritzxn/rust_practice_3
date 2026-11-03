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



}