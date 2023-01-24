use rand::Rng;
use std::thread;

const L3_CACHE_SIZE: usize = 5 * (usize::pow(10, 6));

fn get_random_vec_index(vec_length: usize) -> usize {
    rand::thread_rng().gen_range(0..vec_length)
}

/* Load instructions that misses in the cache are expensive
so I allocate a chunk of memory that is bigger than the l3 cache size
and hop around in this memory */
fn run_heater() {
    let array_length = L3_CACHE_SIZE / std::mem::size_of::<i32>();
    let burner_array: Vec<u32> = (0..array_length).map(|n| n as u32).collect();
    loop {
        let rand_index = get_random_vec_index(array_length);
        let _ = burner_array[rand_index];
    }
}

fn main() {
    // Threads to strain the cpu a little bit more
    thread::spawn(run_heater);
    run_heater();
}
