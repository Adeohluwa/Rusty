

fn main() {
    
    // let predicate: bool = true;
    // let x: f64 = 5.2;
    // println!("The value of x is {x}", );
    // println!("The value of the predicate is {predicate}");

    println!("1 +2 is {}",1u32 + 2);
    println!("1 - 2 is {}",1i32 - 2);
    // println!("1 - 2 is {}",1u32 - 2); wont compile

    // {:?} compound data types require 
    let pair: (i32, bool,f64);
    pair = (1,true,43.5);
    // tuple indexing
    println!("hey {:?}", pair.2);

    // tuple unpacking
    let class: (&str,&str,&str) = ("bayo","tayo","layo");
    let (_student1,_student2,_student3) = class;
    println!("{} is student 1",class.0);


    // Array type & index
    let _a: [i32;5];
    let a = [1,2,3,4,5];
    println!("{} is the first element of the a array",a[0]);


    // Out of bounds access

    use std::io;

    let b: [i32;5];
    let b = [1,2,3,4,5];

    println!("Please enter an array index");
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = b[index];
    println!("The value of the element at index{index} is {element}");

}