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
fn conquer(arr: Vec<u64>) -> Vec<u64> {
    if arr.len() <= 1 {
        return arr;
    }

    let mid = arr.len() / 2;

    let left = conquer(arr[..mid].to_vec());
    let right = conquer(arr[mid..].to_vec());

    let mut res = divide(left, right);
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
    while i < left.len() {
        result.push(left[i]);
        i += 1;
    }
    while j < right.len() {
        result.push(right[j]);
        j += 1;
    }
    result
}
