use std::{ io, thread, time::Duration };
pub struct metric {
    pub sortedArray: [(&'static str, u64); 30],
    pub iterations: [[(&'static str, u64); 30]; 30],
    pub time_taken: String,
}

pub struct AppState {
    pub selected_index: usize,
    pub selected: bool,
    pub array: [(&'static str, u64); 30],
    pub metric: metric,
}
impl AppState {
    pub fn new() -> Self {
        Self {
            selected_index: 0,
            selected: false,
            array: generate_array::generate(),
            metric: metric::new(),
        }
    }

    pub fn next(&mut self) {
        if self.selected == true {
            self.selected = false;
        }
        self.selected_index = (self.selected_index + 1) % 10;
    }

    pub fn previous(&mut self) {
        if self.selected == true {
            self.selected = false;
        }
        if self.selected_index == 0 {
            self.selected_index = 9;
        } else {
            self.selected_index -= 1;
        }
    }
    pub fn submit(&mut self) {
        match self.selected_index {
            0 => {
                let metric: metric = selection_sort::sort(self);
                for x in 0..metric.iterations.len() {
                    if metric.iterations[x][0] == ("E", 0) {
                        break;
                    } else {
                        self.array = metric.iterations[x];
                    }
                }
                // self.array = metric.sortedArray;
            }
            9 => {
                self.array = generate_array::generate();
            }
            _ => {}
        }

        if self.selected == true {
            self.selected = false;
        } else {
            self.selected = true;
        }
    }
}
impl metric {
    pub fn new() -> Self {
        Self {
            sortedArray: [("0", 0); 30],
            iterations: [[("E", 0); 30]; 30],
            time_taken: "null".to_string(),
        }
    }
}

pub mod generate_array;
pub mod selection_sort;
// pub mod bubble_sort;
// pub mod insertion_sort;
// pub mod merge_sort;
// pub mod quick_sort;
// pub mod heap_sort;
// pub mod counting_sort;
// pub mod radix_sort;
// pub mod bucket_sort;
