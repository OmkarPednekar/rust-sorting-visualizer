use std::{ io, thread, time::Duration };
use tui::style::Color;
pub struct Metric {
    pub sortedArray: [(&'static str, u64); 20],
    pub iterations: [[(&'static str, u64); 20]; 20],
    pub time_taken: String,
}

pub struct AppState {
    pub selected_index: usize,
    pub selected: bool,
    pub array: [(&'static str, u64); 20],
    pub metric: Metric,
    pub curr_index: usize,
    pub theme: Color,
}
impl AppState {
    pub fn new() -> Self {
        Self {
            selected_index: 0,
            selected: false,
            array: generate_array::generate(),
            metric: Metric::new(),
            curr_index: 0,
            theme: Color::Rgb(253, 93, 115),
        }
    }
    pub fn change_theme(&mut self) {
        if self.theme == Color::Rgb(253, 93, 115) {
            self.theme = Color::Rgb(0, 63, 145);
        } else {
            self.theme = Color::Rgb(253, 93, 115);
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
    pub fn right(&mut self) {
        if
            self.curr_index != self.array.len() - 2 &&
            self.selected_index != 9 &&
            self.selected != false
        {
            self.curr_index += 1;
        }
    }
    pub fn left(&mut self) {
        if self.curr_index != 0 && self.selected_index != 9 && self.selected != false {
            self.curr_index -= 1;
        }
    }
    pub fn submit(&mut self) {
        match self.selected_index {
            0 => {
                let metric: Metric = selection_sort::sort(self);
                self.array = metric.sortedArray;
                self.metric = metric;
            }
            9 => {
                self.metric = Metric::new();
                self.array = generate_array::generate();
                self.curr_index = 0;
            }
            _ => {}
        }

        if self.selected == true {
            self.selected = false;
        } else {
            if self.curr_index == 0 && self.selected_index != 9 {
                self.curr_index += 1;
            }
            self.selected = true;
        }
    }
}
impl Metric {
    pub fn new() -> Self {
        Self {
            sortedArray: [("", 0); 20],
            iterations: [[("", 0); 20]; 20],
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
