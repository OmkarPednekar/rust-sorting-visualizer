use crate::{ Metric, AppState };
pub fn sort(app_state: &AppState) -> Metric {
    let mut metric_sent = Metric::new();
    let mut arr = app_state.array;
    let mut values: Vec<u64> = app_state.array
        .iter()
        .map(|&(_, value)| value)
        .collect();

    for i in 0..values.len() - 1 {
        for j in 0..values.len() - 1 - i {
            if values[j] > values[j + 1] {
                values.swap(j, j + 1);
            }
        }

        metric_sent.iterations[i] = arr
            .iter()
            .enumerate()
            .map(|(i, &(label, _))| (label, values[i]))
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
    }
    metric_sent
}
