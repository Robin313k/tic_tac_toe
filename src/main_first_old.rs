use clearscreen::clear;
use std::{io, process};

fn main() {
    let mut field = [[0, 0, 0], [0, 0, 0], [0, 0, 0]];

    let player_one = 1;
    let player_two = 2;

    let mut player_one_beginns = true;

    loop {
        if player_one_beginns {
            change_slot(&mut field, player_one);
            check_for_win(&mut field, player_one);
            player_one_beginns = false;
        } else {
            change_slot(&mut field, player_two);
            check_for_win(&mut field, player_two);
            player_one_beginns = true;
        }
        check_for_zero(&field)
    }
    display_game(&field)
}

fn display_game(array: &[[i32; 3]; 3]) {
    let help_scale = [0, 1, 2];
    clear().expect("failed to clear screen");
    for e in 0..array.len() {
        if e == 0 {
            println!(" {:?} x", help_scale);
        }
        print!("{}", e);
        println!("{:?}", array[e]);
    }
    println!("y");
}

fn check_for_zero(array: &[[i32; 3]; 3]) {
    let mut playable = true;
    for element in array {
        if element.contains(&0) {
            playable = true;
            break;
        } else {
            playable = false;
        }
    }
    if playable == false {
        display_game(array);
        println!("");
        println!("Draw!");
        process::exit(0);
    }
}

fn change_slot(array: &mut [[i32; 3]; 3], player: i32) {
    loop {
        display_game(array);
        println!("");
        println!("Player {} is now playing!", player);

        let mut x = String::new();

        println!("Input x:");
        io::stdin().read_line(&mut x).expect("failed to read line");

        let x: usize = match x.trim().parse() {
            // transforms String to Int
            Ok(num) => num,
            Err(_) => continue,
        };

        if 0 > x || x > 2 {
            continue;
        }

        let mut y = String::new();

        println!("Input y:");
        io::stdin().read_line(&mut y).expect("failed to read line");

        let y: usize = match y.trim().parse() {
            // transforms String to Int
            Ok(num) => num,
            Err(_) => continue,
        };

        if 0 > y || y > 2 {
            continue;
        }

        if array[y][x] == 0 {
            array[y][x] = player;
        } else {
            continue;
        }
        break;
    }
}

fn check_for_win(array: &mut [[i32; 3]; 3], player: i32) {
    // Check horizontal wins
    for hor in 0..3 {
        if array[hor][0] == player && array[hor][1] == player && array[hor][2] == player {
            display_game(array);
            println!("");
            println!("Player {} wins!", player);
            process::exit(0);
        }
    }

    // Check vertical wins
    for vert in 0..3 {
        if array[0][vert] == player && array[1][vert] == player && array[2][vert] == player {
            display_game(array);
            println!("");
            println!("Player {} wins!", player);
            process::exit(0);
        }
    }

    // Check diagonal wins
    if (array[0][0] == player && array[1][1] == player && array[2][2] == player)
        || (array[0][2] == player && array[1][1] == player && array[2][0] == player)
    {
        display_game(array);
        println!("");
        println!("Player {} wins!", player);
        process::exit(0);
    }
}
