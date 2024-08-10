use crate::{ metric, AppState };
pub fn sort(app_state: &AppState) -> metric {
    let mut metric_sent = metric::new();
    let mut arr = app_state.array;
    let mut values: Vec<u64> = app_state.array
        .iter()
        .map(|&(_, value)| value)
        .collect();

    for i in 0..values.len() - 1 {
        let mut minindex = i;
        for j in i + 1..values.len() {
            if values[minindex] > values[j] {
                minindex = j;
            }
        }
        values.swap(i, minindex);

        let intermediate_state: Vec<(&'static str, u64)> = app_state.array
            .iter()
            .enumerate()
            .map(|(inde, &(_, _))| ("R", values[inde]))
            .collect();

        // Convert Vec to fixed-size array
        let intermediate_array: [(&'static str, u64); 30] = intermediate_state
            .try_into()
            .expect("Conversion to fixed-size array failed");
        metric_sent.iterations[i] = intermediate_array;
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
