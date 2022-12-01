use std::fs;

fn main() {
    let input = fs::read_to_string("./src/input.txt").unwrap();
    let input = input.split("\r\n").collect::<Vec<&str>>();
    let mut main_arr: Vec<Vec<&str>> = Vec::new();
    let mut sub_arr: Vec<&str> = Vec::new();

    for i in input {
        if i == "" {
            main_arr.push(sub_arr.clone());
            sub_arr.clear();
        } else {
            sub_arr.push(i);
        }
    }

    let mut sum_arr: Vec<i32> = Vec::new();
    let mut temp_val: i32 = 0;

    for arr in main_arr {
        for v in arr {
            temp_val += v.parse::<i32>().expect("Couldn't parse string");
        }
        sum_arr.push(temp_val.clone());
        temp_val = 0;
    }

    let mut highest = 0;
    for v in sum_arr {
        if v > highest {
            highest = v;
        }
    }
    println!("{}", highest);
}
