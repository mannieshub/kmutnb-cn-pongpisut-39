fn main() {
    // split word
    let animal = "this cat this bat this rat";
    let mut word: Vec<&str> = animal.split(" ").collect();

    // word filter
    check_unique(&mut word);

    // count
    let count = word.len();

    // print
    println!("Number of unique words: {}", count);
    println!("Unique words: {:?}", word);
}

fn check_unique(word: &mut Vec<&str>) {
    let mut i = 0;let mut j = i + 1;
    while i < word.len() {
        while j < word.len() {
            if word[i] == word[j] {
                word.remove(j);
            } else {
                j += 1;
            }
        }
        i += 1;
    }
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

