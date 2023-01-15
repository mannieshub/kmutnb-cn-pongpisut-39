fn main() {
//split word
    let animal = "this cat this bat this rat";
    let mut word: Vec<&str> = animal.split(" ").collect();
    println!("{:?}", word);
//word filter
    word.sort_unstable();
    word.dedup();
//count
    let count = word.len();
//test

//print
    println!("Number of unique words: {}", count);
    println!("Unique words: {:?}", word);
}

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn test_sum() {
    let animal = "this cat this bat this rat";
    let mut word: Vec<&str> = animal.split(" ").collect();

    word.sort_unstable();
    word.dedup();

    let mut count = word.len();
    assert_eq!(count,4);    
    }
}

