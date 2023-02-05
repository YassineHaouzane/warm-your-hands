use clap::Parser;

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

 
pub fn parse() -> CpuOptions {
    let parsed_cpu_options = CmdOptions::parse();
    let cache_size = parsed_cpu_options.cache_size.unwrap_or_else(|| 4);
    CpuOptions { cache_size, max_temp: parsed_cpu_options.max_temp }
}
