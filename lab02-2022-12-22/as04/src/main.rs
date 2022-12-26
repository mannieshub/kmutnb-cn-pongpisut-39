use std::io;
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input: i32 = input.trim().parse().expect("Enable number : ");
    let mut num=input;
    let mut one=true; 
    print!("{} = ", input);
    for i in 2..{
        while num%i==0 {
            if one{
                one=false;
            }
            else{
                print!("*");
            }
            print!("{}",i);
            num/=i;
        }
        if num==1{
            break;
        }
    }
}