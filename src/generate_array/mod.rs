use rand::prelude::*;
pub fn generate() -> [(&'static str, u64); 20] {
    let mut rng = thread_rng();
    let mut arr = [("", 0); 20];
    for x in 0..20 {
        let val = rng.gen_range(1..50);
        let index = format!("");
        let index_str = Box::leak(index.into_boxed_str());

        arr[x] = (index_str, val);
    }

    arr
}
