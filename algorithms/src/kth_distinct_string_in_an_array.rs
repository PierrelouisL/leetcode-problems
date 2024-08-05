/*
A distinct string is a string that is present only once in an array.

Given an array of strings arr, and an integer k, return the kth distinct string present in arr. If there are fewer than k distinct strings, return an empty string "".

Note that the strings are considered in the order in which they appear in the array.

Easy Problem

Runtime : 4ms Beats 94.74%
Memory: 2.22 MB beats 52.63%
*/
pub mod kth_distinct_string_in_an_array {
    use std::collections::HashMap;

    pub fn kth_distinct(arr: Vec<String>, k: i32) -> String {
        let mut global_map: HashMap<&str, bool> = HashMap::new();
        for s in &arr {
            if global_map.insert(s.as_str(), true).is_some() {
                global_map.insert(s.as_str(), false);
            }
        }
        let mut cpt = 1;
        for st in &arr {
            if global_map[st.as_str()] {
                if cpt == k {
                    return st.clone();
                }
                cpt += 1;
            }
        }
        return String::new();
    }
}