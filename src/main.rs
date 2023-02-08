use std::process::Command;
use std::{thread, time};
use chrono::prelude::*;

fn main() {
    loop {
        let time = get_time();
        let volume = format_volume(get_volume());

        let print = format!(" {} | {} ", time, volume);
        let _ = Command::new("xsetroot").args(["-name", &print]).spawn();
        let second = time::Duration::from_secs(1);
        thread::sleep(second);
    }
}

fn format_volume(volume: i16) -> String {
    if volume == 0 {
        return format!(" {}", volume)
    }
    else if volume < 50 {
        return format!(" {}", volume);
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

fn get_time() -> String {
    let local = Local::now();
    return format!("{} {} {} | {}:{}", 
        format_time(local.day()), 
        format_time(local.month()), 
        local.year(), 
        format_time(local.hour()), 
        format_time(local.minute()));
}

fn format_time(number: u32) -> String {
    if number < 2 {
        return format!("0{}", number);
    }

    return number.to_string();
}
