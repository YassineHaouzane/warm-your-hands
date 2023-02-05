use clap::Parser;
use cache_size::l3_cache_size;

/// Program to warm your hands through cpu heat
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct CmdOptions {
    /// CPU temperature to be reached
    #[arg(short, long, default_value_t = 60.0)]
    max_temp: f32,
    #[arg(short, long)]
    /// L3 cache size in MB [fallback: 4]
    cache_size: Option<usize>,
}

pub struct CpuOptions {
    pub max_temp: f32,
    pub cache_size: usize,
}

fn print_err_then_quit(s: &str) -> usize {
    eprintln!("[error] ${}", s);
    std::process::exit(1)
}

fn fallback_cpu_cache_size() -> usize {
    match l3_cache_size() {
        // bytes -> mb
        Some(size) => size / 1000000,
        None => print_err_then_quit("couldn't get cpu information please provide l3_cache size"),
    }
}

pub fn parse() -> CpuOptions {
    let parsed_cpu_options = CmdOptions::parse();
    let cache_size = parsed_cpu_options
        .cache_size
        .unwrap_or_else(fallback_cpu_cache_size);
    CpuOptions {
        cache_size,
        max_temp: parsed_cpu_options.max_temp,
    }
}
