use std::fs;

fn main() {
    let power = get_power();

    if power == 0.0 {
        println!("⚡️ AC");
        return;
    }

    println!("{:.2} W", power);
}

fn get_power() -> f64 {
    let power_filepath = "/sys/class/power_supply/BAT0/power_now";

    let contents =
        fs::read_to_string(power_filepath).expect("Something went wrong when reading power file");
    let trimmed_contents: f64 = contents.trim().parse().unwrap();
    let micro_watts = trimmed_contents;
    let power = micro_watts / 1000000.;

    return power;
}

