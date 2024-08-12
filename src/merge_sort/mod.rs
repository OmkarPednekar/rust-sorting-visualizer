use crate::{ Metric, AppState };
use std::cmp::max;

pub fn sort(app_state: &AppState) -> Metric {
    let mut metric_sent = Metric::new();
    let mut arr = app_state.array;
    let mut values: Vec<u64> = app_state.array
        .iter()
        .map(|&(_, value)| value)
        .collect();
    let mut curr_iteration: usize = 0;
    // let mut sorted_arr = conquer(values.clone(), curr_iteration, metric_sent.iterations, arr);
    let sorted_arr = conquer(values);
    metric_sent.sortedArray = arr
        .iter()
        .enumerate()
        .map(|(i, &(label, _))| (label, sorted_arr[i]))
        .collect::<Vec<_>>()
        .try_into()
        .expect("Array conversion failed");
    metric_sent.iterations[1] = metric_sent.sortedArray
        .iter()
        .enumerate()
        .map(|(i, &(label, _))| (label, sorted_arr[i]))
        .collect::<Vec<_>>()
        .try_into()
        .expect("Array conversion failed");

    metric_sent
}
fn conquer(
    arr: Vec<u64>
    // curr_iteration: usize,
    // iterations: [[(&'static str, u64); 20]; 20],
    // array: [(&'static str, u64); 20]
) -> Vec<u64> {
    // let mut curr_iter = [[("", 0); 20]; 20];
    if arr.len() <= 1 {
        return arr;
    }

    // Find the middle index
    let mid = arr.len() / 2;

    // Recursively split and sort both halves
    let left = conquer(arr[..mid].to_vec());
    let right = conquer(arr[mid..].to_vec());

    // Merge the sorted halves
    let mut res = divide(left, right);

    // curr_iter[curr_iteration] = array
    //     .iter()
    //     .enumerate()
    //     .map(|(i, &(label, _))| (label, res[i]))
    //     .collect::<Vec<_>>()
    //     .try_into()
    //     .expect("Array conversion failed");
    res
}
fn divide(left: Vec<u64>, right: Vec<u64>) -> Vec<u64> {
    let mut result: Vec<u64> = Vec::with_capacity(left.len() + right.len());
    let mut i = 0;
    let mut j = 0;

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            result.push(left[i]);
            i += 1;
        } else {
            result.push(right[j]);
            j += 1;
        }
    }

    // Add any remaining elements from the left array
    while i < left.len() {
        result.push(left[i]);
        i += 1;
    }

    // Add any remaining elements from the right array
    while j < right.len() {
        result.push(right[j]);
        j += 1;
    }

    result
}
