use rand::Rng;
use std::{thread, time};
use systemstat::platform::PlatformImpl;
use systemstat::{Platform, System};
use crate::cmdparser::CpuOptions;

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

fn get_ceil_byte_size(cache_size: usize) -> usize {
    // + 1 to get the size in byte above the l3 cache size
   (cache_size + 1) * usize::pow(10, 6)
}

/* Load instructions that lead to cache misses are expensive
so I allocate a chunk of memory that is bigger than the l3 cache size
and hop around in this memory */
pub fn run_heater(cpu_options: CpuOptions) {
    let CpuOptions { max_temp, cache_size } = cpu_options;
    let cache_byte_size = get_ceil_byte_size(cache_size);
    let sys = System::new();
    let array_length = cache_byte_size / std::mem::size_of::<i32>();
    let burner_array: Vec<usize> = (0..array_length).collect();
    /*  An infinite loop seems to be working aswell but this heats up the cpu
    way faster */

    loop {
        let cpu_temp = get_cpu_temp(&sys);
        if cpu_temp < max_temp {
            println!("We heat ! ${}", cpu_temp);
            let rand_index = get_random_vec_index(array_length);
            let _ = burner_array[rand_index];
        } else {
            println!("Don't heat !!! ${}", cpu_temp);
            thread::sleep(time::Duration::from_secs(1));
        }
    }
}
