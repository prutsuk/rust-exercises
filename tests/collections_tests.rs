use rust_exercises::collections::*;

#[test]
fn binary_search_found() {
    let data = vec![1, 3, 5, 7, 9, 11];
    assert_eq!(binary_search(&data, &5), Some(2));
    assert_eq!(binary_search(&data, &1), Some(0));
    assert_eq!(binary_search(&data, &11), Some(5));
}

#[test]
fn binary_search_not_found() {
    let data = vec![1, 3, 5, 7, 9];
    assert_eq!(binary_search(&data, &2), None);
    assert_eq!(binary_search(&data, &10), None);
}

#[test]
fn binary_search_empty() {
    let data: Vec<i32> = vec![];
    assert_eq!(binary_search(&data, &1), None);
}

#[test]
fn binary_search_single() {
    assert_eq!(binary_search(&[42], &42), Some(0));
    assert_eq!(binary_search(&[42], &0), None);
}

#[test]
fn merge_sort_basic() {
    let mut data = vec![5, 3, 8, 1, 2];
    merge_sort(&mut data);
    assert_eq!(data, vec![1, 2, 3, 5, 8]);
}

#[test]
fn merge_sort_already_sorted() {
    let mut data = vec![1, 2, 3, 4, 5];
    merge_sort(&mut data);
    assert_eq!(data, vec![1, 2, 3, 4, 5]);
}

#[test]
fn merge_sort_reverse() {
    let mut data = vec![5, 4, 3, 2, 1];
    merge_sort(&mut data);
    assert_eq!(data, vec![1, 2, 3, 4, 5]);
}

#[test]
fn merge_sort_empty_and_single() {
    let mut empty: Vec<i32> = vec![];
    merge_sort(&mut empty);
    assert_eq!(empty, Vec::<i32>::new());

    let mut single = vec![42];
    merge_sort(&mut single);
    assert_eq!(single, vec![42]);
}

#[test]
fn merge_sort_duplicates() {
    let mut data = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
    merge_sort(&mut data);
    assert_eq!(data, vec![1, 1, 2, 3, 3, 4, 5, 5, 6, 9]);
}

#[test]
fn dedup_sorted_basic() {
    assert_eq!(dedup_sorted(&[1, 1, 2, 3, 3, 3, 4]), vec![1, 2, 3, 4]);
}

#[test]
fn dedup_sorted_no_dupes() {
    assert_eq!(dedup_sorted(&[1, 2, 3]), vec![1, 2, 3]);
}

#[test]
fn dedup_sorted_empty() {
    assert_eq!(dedup_sorted::<i32>(&[]), Vec::<i32>::new());
}

#[test]
fn dedup_sorted_all_same() {
    assert_eq!(dedup_sorted(&[5, 5, 5, 5]), vec![5]);
}

#[test]
fn flatten_basic() {
    let nested = vec![vec![1, 2], vec![3], vec![4, 5, 6]];
    assert_eq!(flatten(nested), vec![1, 2, 3, 4, 5, 6]);
}

#[test]
fn flatten_empty() {
    let nested: Vec<Vec<i32>> = vec![];
    assert_eq!(flatten(nested), Vec::<i32>::new());

    let nested_empty: Vec<Vec<i32>> = vec![vec![], vec![]];
    assert_eq!(flatten(nested_empty), Vec::<i32>::new());
}

#[test]
fn group_consecutive_basic() {
    assert_eq!(
        group_consecutive(&[1, 1, 2, 3, 3, 3, 2]),
        vec![vec![1, 1], vec![2], vec![3, 3, 3], vec![2]]
    );
}

#[test]
fn group_consecutive_empty() {
    assert_eq!(group_consecutive::<i32>(&[]), Vec::<Vec<i32>>::new());
}

#[test]
fn group_consecutive_single() {
    assert_eq!(group_consecutive(&[1]), vec![vec![1]]);
}

#[test]
fn unique_preserve_order_basic() {
    assert_eq!(unique_preserve_order(&[3, 1, 4, 1, 5, 3]), vec![3, 1, 4, 5]);
}

#[test]
fn unique_preserve_order_no_dupes() {
    assert_eq!(unique_preserve_order(&[1, 2, 3]), vec![1, 2, 3]);
}

#[test]
fn unique_preserve_order_empty() {
    assert_eq!(unique_preserve_order::<i32>(&[]), Vec::<i32>::new());
}

#[test]
fn transpose_basic() {
    let m = vec![vec![1, 2, 3], vec![4, 5, 6]];
    assert_eq!(
        transpose(&m),
        Some(vec![vec![1, 4], vec![2, 5], vec![3, 6]])
    );
}

#[test]
fn transpose_empty() {
    let m: Vec<Vec<i32>> = vec![];
    assert_eq!(transpose(&m), Some(vec![]));
}

#[test]
fn transpose_ragged_returns_none() {
    let m = vec![vec![1, 2], vec![3]];
    assert_eq!(transpose(&m), None);
}

#[test]
fn transpose_single_row() {
    let m = vec![vec![1, 2, 3]];
    assert_eq!(transpose(&m), Some(vec![vec![1], vec![2], vec![3]]));
}
