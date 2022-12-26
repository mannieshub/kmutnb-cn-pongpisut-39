use std::io;
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input: i32 = input.trim().parse().expect("Enable number : ");

    for i in 1..input+1 { 
        for z in 0..(input-i)+1/2 {
            print!(" "); 
         } 
        for j in 1..i*2 {
           print!("*"); 
        }
        println!("");
    }
}