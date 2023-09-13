
// without mut, reassigning will yield compile time error
// fn main() {
//     let mut x = 5;
//     println!("The value of x is {x}");
//     x = 6;
//     println!("The value of x is {x}");
// }

// compile time immutable 
// const THREE_HOURS_IN_SECONDS: u32 = 60*60*3;


fn main() {
    let x = 5;
    let x = x+1;
    
    // local scope
    {
        let x = x*2;
        println!("The value of x in the inner scope is {x}");
    }
    println!("The value of x is {x}");



    fn add(x: f64, y: f64) -> f64 {
        x+y
    }

    let result = add(2.2,3.3);

    println!("the result of add(2,3) {result}"); 

// remove semicolon for implicit return
    // fn five() -> i32 {
    //     5
    // }

    // println!("{:?}", five());


}