use crate::{ Metric, AppState };
pub fn sort(app_state: &AppState) -> Metric {
    let mut metric_sent = Metric::new();
    let mut arr = app_state.array;
    let mut values: Vec<u64> = app_state.array
        .iter()
        .map(|&(_, value)| value)
        .collect();

    for i in 1..values.len() {
        let key = values[i];
        let mut j = (i as isize) - 1;

        while j >= 0 && key < values[j as usize] {
            values[(j + 1) as usize] = values[j as usize];
            j -= 1;
        }

        values[(j + 1) as usize] = key;

        metric_sent.iterations[i] = arr
            .iter()
            .enumerate()
            .map(|(i, &(label, _))| (label, values[i]))
            .collect::<Vec<_>>()
            .try_into()
            .expect("Array conversion failed");
    }

    metric_sent.sortedArray = arr
        .iter()
        .enumerate()
        .map(|(i, &(label, _))| (label, values[i]))
        .collect::<Vec<_>>()
        .try_into()
        .expect("Array conversion failed");

    metric_sent
}
