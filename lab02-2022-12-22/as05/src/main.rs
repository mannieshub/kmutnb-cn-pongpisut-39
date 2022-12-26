use std::io;
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input: i32 = input.trim().parse().expect("Enable number : ");

    for i in 0..input{
        for j in 0..input{
            if j==0||j==input-1||j==i{
                print!(" X ");
            }
            else{
                print!(" O "); 
            }
        }
        println!("");
    }
}