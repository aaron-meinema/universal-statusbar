use std::process::Command;
use std::{thread, time};
use std::env;
mod time_format;

fn main() {
    //{day month year | hour:minute }
    let args: Vec<String> = env::args().collect();
    let mut option_t = false;
    let mut use_args = false;
    let mut time_argument = "";
    for arg in args.iter() {
        if option_t {
            time_argument = arg;
            option_t = false;
        }
        if arg.eq("-t") {
            option_t = true;
            use_args = true;
        }
    }
    let time_arg = time_argument.split(" ");

    loop {
        let args = time_arg.clone();
        let time = time_format::get_time(args, use_args);
        let volume = format_volume(get_volume());
        let power = format_power(get_power());
        let print = format!(" {} | {} | {}",volume , power, time);
        let _ = Command::new("xsetroot").args(["-name", &print]).spawn();
        let second = time::Duration::from_secs(1);
        thread::sleep(second);
    }
}

fn format_power(power: i16) -> String {
    if power < 8 {
        return format!(" {}", power);
    }
    if power < 30 {
        return format!(" {}", power);
    }
    if power < 60 {
        return format!(" {}", power);
    }
    if power < 80 {
        return format!(" {}", power);
    }


    return format!(" {}", power);
}

fn get_power() -> i16 {
    let args = ["-i", "/org/freedesktop/UPower/devices/battery_BAT0"];
    let command = Command::new("upower").args(args).output().unwrap();
    let full_text = String::from_utf8(command.stdout).unwrap();
    let split = full_text.split("\n");
    for line in split{
        if line.contains("percentage:") {
            let mut percentage = line.replace("percentage:", "").trim().to_string();
            percentage.pop();
            return percentage.parse().unwrap();
        }
    }
    return 0;
}

fn format_volume(volume: i16) -> String {
    if volume == 0 {
        return format!(" {}", volume)
    }
    else if volume < 50 {
        return format!(" {}", volume);
    }

    return format!(" {}", volume);
}

fn get_volume() -> i16 {
    let commands = ["pactl", "list", "sinks"];
    let command = Command::new(commands[0]).args([commands[1], commands[2]]).output().unwrap();
    
    let full_text = String::from_utf8(command.stdout).unwrap();
    let mut lines = full_text.split("\n");
    let line = lines.find(|variable| variable.trim().starts_with("Volume")).unwrap_or("-1");
    let mut split = line.split("/");
    let mut vol = split.find(|variable| variable.contains("%")).unwrap_or("-1").trim().to_string();
    if vol.eq("-1") {
        println!("could not find volume with '{} {} {}', output: \n\n{}", 
        commands[0], commands[1], commands[2], full_text);
        return -1;
    }
    vol.pop();
    
    return vol.parse().unwrap();
}

