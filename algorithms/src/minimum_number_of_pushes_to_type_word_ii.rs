/*
You are given a string word containing lowercase English letters.

Telephone keypads have keys mapped with distinct collections of lowercase English letters, which can be used to form words by pushing them. For example, the key 2 is mapped with ["a","b","c"], we need to push the key one time to type "a", two times to type "b", and three times to type "c" .

It is allowed to remap the keys numbered 2 to 9 to distinct collections of letters. The keys can be remapped to any amount of letters, but each letter must be mapped to exactly one key. You need to find the minimum number of times the keys will be pushed to type the string word.

Return the minimum number of pushes needed to type word after remapping the keys.

An example mapping of letters to keys on a telephone keypad is given below. Note that 1, *, #, and 0 do not map to any letters.

Medium problem: https://leetcode.com/problems/minimum-number-of-pushes-to-type-word-ii/description/?envType=daily-question&envId=2024-08-06
*/
pub mod kth_distinct_string_in_an_array {
    pub fn minimum_pushes(word: String) -> i32 {
        if word.is_empty() {
            return 0;
        }
        let mut letter_array: [(u8, i32); 26] = [(0,0); 26];
        let mut cpt: u8 = 0;
        for i in letter_array.as_mut() {
            *i = (cpt + 'a' as u8, 0);
            cpt += 1;
        } 
        for letter in word.as_bytes() {
            letter_array[(letter - ('a' as u8)) as usize].1 += 1;
        }
        letter_array.sort_by_key(|a| a.1);
        letter_array.reverse();
        let mut sum = 0;
        let mut next_key_i = 0;
        for j in letter_array {
            if next_key_i < 8 {
                sum += j.1;
            } else if next_key_i < 16 {
                sum += j.1 * 2;
            } else if next_key_i < 24 {
                sum += j.1 * 3;
            } else {
                sum += j.1 * 4;
            }
            next_key_i +=1;
        }
        sum
    }
        
}