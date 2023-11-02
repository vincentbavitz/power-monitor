use std::fs;
use std::path::Path;

fn main() {
    let power = get_power();

    if power == 0.0 {
        println!("⚡️ AC");
    } else {
        println!("{:.2} W", power);
    }
}

fn get_power() -> f64 {
    let power_filepath_bat0 = "/sys/class/power_supply/BAT0/power_now";
    let power_filepath_bat1 = "/sys/class/power_supply/BAT1/power_now";

    // Try BAT0 first
    if let Ok(power) = read_power_file(power_filepath_bat0) {
        return power;
    }

    // Try BAT1 next
    if let Ok(power) = read_power_file(power_filepath_bat1) {
        return power;
    }

    // If both fail, return 0.0
    0.0
}

fn read_power_file(filepath: &str) -> Result<f64, ()> {
    if !Path::new(filepath).exists() {
        return Err(());
    }

    let contents = match fs::read_to_string(filepath) {
        Ok(contents) => contents,
        Err(_) => return Err(())
    };

    let trimmed_contents: f64 = match contents.trim().parse() {
        Ok(num) => num,
        Err(_) => return Err(())
    };

    let micro_watts = trimmed_contents;
    let power = micro_watts / 1_000_000.;

    Ok(power)
}
