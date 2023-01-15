fn main() {
    let number = 100;
    let sn = sum(number);
    println!("The sum of the first {} natural numbers is: {}", number, sn);
}

fn sum(number: i32) -> i32 {
    (number*(1+number))/2
}

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn test_sum() {
    let number = 100;
    let sn = sum(number);
    assert_eq!(sn,5050);
   }
}