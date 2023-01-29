mod heater;
use heater::run_heater;
mod cmdparser;

fn main() {
    let cpu_options = cmdparser::parse();
    let max_temp = cpu_options.max_temp;
    run_heater(max_temp);
}
