// operations and printing in rust
// fn main() {
//     let x:i32 = 5;
//     let mut y:i32 = 10;
//     println!("x = {}, y = {}",x,y);
//     y += 1;
//     println!("Now y = {}",y);
// }


//taking input in rust
// use std::io;
// fn main(){
//     let mut input:String = String::new();
//     println!("Enter your name: ");
//     io::stdin().read_line( &mut input).unwrap();
//     println!("Hello, {}!", input.trim());
// }

fn main(){
    let n: i32 = 5;

    if n%2 == 0 {
        println!("Even");
    } else {
        println!("Odd");
    }

    for i in 0..n {
        println!("i = {}", i);
    }
}