
use std::time::Instant;

fn same_necklace(word1: &str , word2: &str) {
    
    let now = Instant::now();

    let same = calculate_same_necklace(word1, word2);

    println!("|{0} ns| \"{1}\" \"{2}\" => {3}", now.elapsed().as_nanos(), word1, word2, same);
}

fn calculate_same_necklace(word1: &str, word2: &str) -> bool {
    
    if word1.eq(word2) || word1.len() != word2.len() {
        return true;
    }

    let mut str1 = String::from(word1);

    for _ in word1.chars() {
        shift_characters(&mut str1);

        if word2.eq(&str1) {
            return true;
        }
    }

    return false;
}

fn shift_characters(word: &mut String) {
    if word.len() != 0 {
        let character = word.remove(0);
        word.push(character);
    }
}



fn main() {
    println!("\nhttps://www.reddit.com/r/dailyprogrammer challenge 383 (easy)\n");
    same_necklace("nicole", "icolen");
    same_necklace("nicole", "lenico");
    same_necklace("nicole", "coneli");
    same_necklace("aabaaaaabaab", "aabaabaabaaa");
    same_necklace("abc", "cba");
    same_necklace("xxyyy", "xxxyy");
    same_necklace("xyxxz", "xxyxz");
    same_necklace("x", "x");
    same_necklace("x", "xx");
    same_necklace("x", "");
    same_necklace("", "");
}
