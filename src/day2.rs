use std::vec;

use crate::utils::{read_file, smallest};

pub fn day2a() {
    let contents = read_file("inputs/day2.txt".to_string());
    let mut area: i64 = 0;

    // Loop
    for line in contents.lines() {
        let parts: Vec<&str> = line.split('x').collect();
        let l = parts[0].parse::<i64>().unwrap();
        let w = parts[1].parse::<i64>().unwrap();
        let h = parts[2].parse::<i64>().unwrap();

        area += 2 * l * w + 2 * w * h + 2 * h * l;

        area += smallest(vec![l * w, w * h, h * l]);
    }

    // print distance
    println!("Paper to use: {}", area);
}

pub fn day2b() {
    let contents = read_file("inputs/day2.txt".to_string());
    let mut lenght: i64 = 0;

    // Loop
    for line in contents.lines() {
        let parts: Vec<&str> = line.split('x').collect();
        let l = parts[0].parse::<i64>().unwrap();
        let w = parts[1].parse::<i64>().unwrap();
        let h = parts[2].parse::<i64>().unwrap();

        let mut sorted_sides = vec![l,w,h];
        sorted_sides.sort_unstable();

        let side1 = sorted_sides[0];
        let side2 = sorted_sides[1];

        lenght += side1*2+side2*2;
        lenght += l*w*h;
    }

    // print distance
    println!("Ribbon lenght: {}", lenght);
}
