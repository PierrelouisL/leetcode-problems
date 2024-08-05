use algorithms::kth_distinct_string_in_an_array::*;

#[test]
fn kth_distinct_string_in_an_array_1() {
    assert_eq!(kth_distinct_string_in_an_array::kth_distinct(vec!["d".to_string(),"b".to_string(),"c".to_string(),"b".to_string(),"c".to_string(),"a".to_string()], 2), String::from("a"));
}

#[test]
fn kth_distinct_string_in_an_array_2() {
    assert_eq!(kth_distinct_string_in_an_array::kth_distinct(vec!["aaa".to_string(),"aa".to_string(),"a".to_string()], 1), String::from("aaa"));
}
#[test]
fn kth_distinct_string_in_an_array_3() {
    assert_eq!(kth_distinct_string_in_an_array::kth_distinct(vec!["a".to_string(),"b".to_string(),"a".to_string()], 3), String::new());
}
