use rand::prelude::*;
pub fn generate() -> [(&'static str, u64); 30] {
    let mut rng = thread_rng();
    let mut arr = [("", 0); 30];
    for x in 0..30 {
        let index = format!("{}", x);
        let index_str = Box::leak(index.into_boxed_str());
        arr[x] = (index_str, rng.gen_range(1..50));
    }

    arr
}
