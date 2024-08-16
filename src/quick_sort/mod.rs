use crate::{ Metric, AppState };

pub fn sort(app_state: &AppState) -> Metric {
    let mut metric_sent = Metric::new();
    let mut arr = app_state.array;
    let mut values: Vec<u64> = app_state.array
        .iter()
        .map(|&(_, value)| value)
        .collect();
    let mut counter: usize = 0;
    quick_sort(&mut values, &mut metric_sent, &mut arr, &mut counter);
    metric_sent.iterations[1] = arr
        .iter()
        .enumerate()
        .map(|(index, &(label, _))| {
            if index < values.len() { (label, values[index]) } else { (label, 0) }
        })
        .collect::<Vec<_>>()
        .try_into()
        .expect("Array conversion failed");
    metric_sent.sortedArray = arr
        .iter()
        .enumerate()
        .map(|(i, &(label, _))| (label, values[i]))
        .collect::<Vec<_>>()
        .try_into()
        .expect("Array conversion failed");

    metric_sent
}

fn quick_sort(
    values: &mut [u64],
    metric_sent: &mut Metric,
    arr: &mut [(&'static str, u64); 20],
    counter: &mut usize
) {
    if values.len() <= 1 {
        return;
    }

    let pivot_index = partition(values, metric_sent, arr, counter);

    quick_sort(&mut values[0..pivot_index], metric_sent, arr, counter);
    quick_sort(&mut values[pivot_index + 1..], metric_sent, arr, counter);
}

fn partition(
    values: &mut [u64],
    metric_sent: &mut Metric,
    arr: &mut [(&'static str, u64); 20],
    counter: &mut usize
) -> usize {
    let l = values.len();
    let pivot = values[l - 1];
    let mut i = 0;
    *counter += 1;
    for j in 0..l - 1 {
        if values[j] <= pivot {
            values.swap(i, j);
            i += 1;
        }
    }

    values.swap(i, l - 1);
    metric_sent.iterations[*counter] = arr
        .iter()
        .enumerate()
        .map(|(index, &(label, _))| {
            if index < values.len() { (label, values[index]) } else { (label, 0) }
        })
        .collect::<Vec<_>>()
        .try_into()
        .expect("Array conversion failed");
    // Update the iterations for the sorting visualizer

    i
}
