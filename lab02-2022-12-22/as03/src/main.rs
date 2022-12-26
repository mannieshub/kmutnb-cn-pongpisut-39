use std::io;
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");


    let input: i32 = input.trim().parse().expect("Enable number : ");
    let mut n=input;

    for i in 1..input+1 {
        for k in 0..i {
            print!(" ");
        }
        for j in 1..n*2{
           print!("*"); 
        }

        println!("");
        n=n-1;
    }
}