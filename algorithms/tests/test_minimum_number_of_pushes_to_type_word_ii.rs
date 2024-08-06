use algorithms::minimum_number_of_pushes_to_type_word_ii::*;
use kth_distinct_string_in_an_array::minimum_pushes;

#[test]
fn test1() {
    assert_eq!(5, minimum_pushes("abcde".to_string()));
}

#[test]
fn test2() {
    assert_eq!(12, minimum_pushes("xyzxyzxyzxyz".to_string()));
}
#[test]
fn test3() {
    assert_eq!(24, minimum_pushes("aabbccddeeffgghhiiiiii".to_string()));
}
#[test]
fn test4() {
    assert_eq!(39, minimum_pushes("ajqjtbjhczpakocxjrsugawef".to_string()));
}
#[test]
fn test5() {
    assert_eq!(37, minimum_pushes("alnywlxhptelldbclzccqrhvi".to_string()));
}