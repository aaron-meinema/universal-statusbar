use std::str::Split;
use chrono::prelude::*;


pub fn get_time(arguments: Split<&str>, use_args: bool) -> String {
    if use_args{
        return get_time_with_arguments(arguments);
    }

    return get_time_without_arguments();
}

fn get_time_without_arguments() -> String {
    let local = Local::now();
    return format!("{} {} {} | {}:{}", 
        format_time(local.day()), 
        format_time(local.month()), 
        local.year(), 
        format_time(local.hour()), 
        format_time(local.minute()));
}

fn get_time_with_arguments(arguments: Split<&str>) -> String {
    let local = Local::now();
    let mut time = "".to_string();
    for arg in arguments{
        if arg.contains(":") || arg.contains("|") || arg.contains("-"){
            if arg.len() > 1 {
                if arg.contains(":") {
                    let split = arg.split(":");
                    time = split_time(time, ":", split, local);
                }
                else if arg.contains("|") {
                    let split = arg.split("|");
                    time = split_time(time, "|", split, local);
                }
                else {
                    let split = arg.split("-");
                    time = split_time(time, "-", split, local);
                }

            }
            else {
                time = format!("{} {}", time, arg)
            }
        }
        else {
            time = format!("{} {}", time, get_time_string(arg, local));
        }
    }

    return time;
}

fn split_time(mut time: String, seperator: &str, split: Split<&str>, local: DateTime<Local>) -> String{
    let mut first = true;
    for part in split {
        if !first {
            time = format!("{}{}{}", time, seperator, get_time_string(part, local));
        }
        else {
            time = format!("{} {}", time, get_time_string(part, local));
            first = false;
        }
    }

    return time;
}

fn get_time_string(arg: &str, local: DateTime<Local>) -> String {
    if arg.eq("day"){
        return format_time(local.day());
    }
    else if arg.eq("month"){
        return format_time(local.month());
    }
    else if arg.eq("year"){
        return local.year().to_string();
    }
    else if arg.eq("hour"){
        return format_time(local.hour());
    }
    else if arg.eq("min") || arg.eq("minute"){
        return format_time(local.minute());
    }
    println!("{} is not a valid option for -t", arg);
    return String::from("");
}

fn format_time(number: u32) -> String {
    if number < 10 {
        return format!("0{}", number);
    }

    return number.to_string();
}
