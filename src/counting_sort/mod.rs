use crate::{ Metric, AppState };

pub fn sort(app_state: &AppState) -> Metric {
    let mut metric_sent = Metric::new();
    let mut arr = app_state.array;
    let mut values: Vec<u64> = app_state.array
        .iter()
        .map(|&(_, value)| value)
        .collect();

    // Find the maximum value for counting sort
    let max_val = match values.iter().max() {
        Some(&val) => val as usize,
        None => 0,
    };

    // Initialize the count array
    let mut count_arr: Vec<u64> = vec![0; max_val + 1];

    // Count occurrences of each value
    for &val in &values {
        count_arr[val as usize] += 1;
    }

    // Update the count array for cumulative counts
    for i in 1..=max_val {
        count_arr[i] += count_arr[i - 1];
    }

    // Initialize the output array
    let mut output_arr: Vec<u64> = vec![0; values.len()];

    // Build the output array in reverse order to maintain stability
    let mut counter: usize = 1;
    for i in (0..values.len()).rev() {
        let val = values[i] as usize;
        output_arr[(count_arr[val] as usize) - 1] = val as u64;
        count_arr[val] -= 1;

        // Store intermediate iteration results
        if i < values.len() - 1 {
            metric_sent.iterations[counter] = arr
                .iter()
                .enumerate()
                .map(|(i, &(label, _))| (label, output_arr[i]))
                .collect::<Vec<_>>()
                .try_into()
                .expect("Array conversion failed");
            counter += 1;
        }
    }

    // Store the final sorted array
    metric_sent.sortedArray = arr
        .iter()
        .enumerate()
        .map(|(i, &(label, _))| (label, output_arr[i]))
        .collect::<Vec<_>>()
        .try_into()
        .expect("Array conversion failed");

    metric_sent
}
