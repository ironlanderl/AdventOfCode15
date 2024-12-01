use crate::utils::read_file;

pub fn day1a(){
    let contents = read_file("inputs/day1.txt".to_string());
    let mut floor: i64 = 0;

    // Loop
    for chara in contents.chars(){
        if chara == '('{
            floor += 1;
        }
        else if chara == ')' {
            floor += -1 ;
        }
        else {
            println!("Unrecognized char: {:?}", chara);
        }
    }

    // print distance
    println!("Final floor: {}", floor);
    
}

pub fn day1b(){
    let contents = read_file("inputs/day1.txt".to_string());
    let mut floor: i64 = 0;

    // Loop
    for (index, chara) in contents.chars().enumerate(){
        if chara == '('{
            floor += 1;
        }
        else if chara == ')' {
            floor += -1 ;
        }
        else {
            println!("Unrecognized char: {:?}", chara);
        }

        if floor < 0{
            println!("Entered basement at {}", index + 1);
            return;
        }
    }
}