use crate::{ Metric, AppState };
pub fn sort(app_state: &AppState) -> Metric {
    let mut metric_sent = Metric::new();
    let mut arr = app_state.array;
    let mut values: Vec<u64> = app_state.array
        .iter()
        .map(|&(_, value)| value)
        .collect();
    let mut iter_count: u64 = 0;
    for i in 0..values.len() - 1 {
        let mut minindex = i;
        for j in i + 1..values.len() {
            if values[minindex] > values[j] {
                minindex = j;
            }
            iter_count = iter_count + 1;
        }
        values.swap(i, minindex);
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
