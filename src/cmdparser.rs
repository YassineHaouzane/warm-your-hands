use clap::Parser;

/// Program to warm your hands through cpu heat 
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CpuOptions {
    /// Number of times to greet
   #[arg(short, long, default_value_t = 60.0)]
    pub max_temp: f32,
}


pub fn parse() -> CpuOptions {
    CpuOptions::parse()
}
