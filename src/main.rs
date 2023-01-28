use rand::Rng;
use std::{thread, time};
use systemstat::platform::PlatformImpl;
use systemstat::{Platform, System};

const L3_CACHE_SIZE: usize = 5 * (usize::pow(10, 6));

fn get_random_vec_index(vec_length: usize) -> usize {
    rand::thread_rng().gen_range(0..vec_length)
}

fn get_cpu_temp(p: &PlatformImpl) -> f32 {
    match p.cpu_temp() {
        Ok(cpu) => cpu,
        Err(e) => {
            println!("[error]: couldn't get cpu temperature ${}", e);
            f32::MAX
        }
    }
}

/* Load instructions that misses in the cache are expensive
so I allocate a chunk of memory that is bigger than the l3 cache size
and hop around in this memory */
fn run_heater() {
    let sys = System::new();
    let array_length = L3_CACHE_SIZE / std::mem::size_of::<i32>();
    let burner_array: Vec<u32> = (0..array_length).map(|n| n as u32).collect();
    /*  An infinite loop seems to be working aswell but this heats up the cpu
    way faster */

    loop {
        let cpu_temp = get_cpu_temp(&sys);
        if cpu_temp < 60.0 {
            println!("We heat ! ${}", cpu_temp);
            let rand_index = get_random_vec_index(array_length);
            let _ = burner_array[rand_index];
        } else {
            println!("Don't heat !!! ${}", cpu_temp);
            thread::sleep(time::Duration::from_secs(1));
        }
    }
}

fn main() {
    run_heater();
}
