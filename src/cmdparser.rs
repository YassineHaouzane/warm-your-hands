use clap::Parser;

/// Program to warm your hands through cpu heat 
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CpuOptions {
    /// CPU temperature to be reached 
   #[arg(short, long, default_value_t = 60.0)]
    pub max_temp: f32,
   #[arg(short, long, default_value_t = 4)]
    /// L3 cache size in MB 
    pub cache_size: usize,
}


pub fn parse() -> CpuOptions {
    CpuOptions::parse()
}
