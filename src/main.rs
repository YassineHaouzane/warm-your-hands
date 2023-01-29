mod heater;
mod cmdparser;

fn main() {
    let cpu_options = cmdparser::parse();
    heater::run_heater(cpu_options);
}
