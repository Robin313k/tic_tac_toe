use clearscreen::clear;
use std::{io, process};

fn main() {
    // var for gamefield
    let mut field = [[0, 0, 0], [0, 0, 0], [0, 0, 0]];

    // var for players
    let player_one = 1;
    let player_two = 2;

    // var for startplayer and needed to switch player
    let mut player_one_beginns = true;

    loop {
        // if else statement is responsible for switching out players
        if player_one_beginns {
            display_game(&field);
            change_slot(&mut field, player_one);
            check_for_win(&mut field, player_one);
            player_one_beginns = false;
        } else {
            display_game(&field);
            change_slot(&mut field, player_two);
            check_for_win(&mut field, player_two);
            player_one_beginns = true;
        }
        // check if any slots are unclaimed, if so draw happens, bc no one won
        if check_for_zero(&field) == false {
            break;
        }
    }
    // show final result
    display_game(&field);
    println!("");
    println!("Draw!");





}

// display gamefield in a more appealing way
fn display_game(array: &[[i32; 3]; 3]) {
    // vars for empty, player one and player two
    let x = String::from("[X]");
    let o = String::from("[O]");

    // clear screen so player wont need to scroll
    clear().expect("failed to clear screen");
    // starts process of printing gamefield
    let mut counter = 0;
    for i in 0..3 {
        for j in 0..3 {
            counter = counter + 1;
            match array[i][j] {
                0 => print!("[{}]", counter),
                1 => print!("{}", &x),
                2 => print!("{}", &o),
                _ => continue, // Default case for unexpected values
            };
        }
        println!();
    }
}

// check if draw happened
fn check_for_zero(array: &[[i32; 3]; 3]) -> bool {
    let mut playable = true;
    // check if playable
    for element in array {
        if element.contains(&0) {
            playable = true;
            break;
        } else {
            playable = false;
        }
    }
    // draw happens when the game has no empty slots left
    if playable == false {
        return false;
    } else {
        return true;
    }
}

// change slot from empty to claimed
fn change_slot(array: &mut [[i32; 3]; 3], player: i32) {
    let mut input = String::new();

    let help;

    match player {
        1 => help = String::from("[X]"),
        2 => help = String::from("[O]"),
        _ => help = String::new(),
    }

    // Loop to prevent false input from crashing the game
    loop {
        // Display the game field
        display_game(array);
        println!("");
        println!("Player {} {help} is now playing!", player);

        // Prompt for input
        println!("Input the field you want to claim (1-9):");
        input.clear();  // Clear input from previous loop iteration
        io::stdin().read_line(&mut input).expect("Failed to read line");

        // Try to parse the input as an integer between 1 and 9
        let input: u8 = match input.trim().parse() {
            Ok(num) if num >= 1 && num <= 9 => num,  // Only accept input between 1 and 9
            _ => {
                println!("Invalid input, please enter a number between 1 and 9.");
                continue;
            }
        };

        // Map the input to (x, y) coordinates
        let (x, y) = match input {
            1 => (0, 0),
            2 => (1, 0),
            3 => (2, 0),
            4 => (0, 1),
            5 => (1, 1),
            6 => (2, 1),
            7 => (0, 2),
            8 => (1, 2),
            9 => (2, 2),
            _ => unreachable!(), // This should never be reached
        };

        // Check if the slot is empty and within bounds
        if array[y as usize][x as usize] == 0 {
            array[y as usize][x as usize] = player;
            break;  // Exit the loop when a valid move is made
        } else {
            println!("Slot already taken, choose another one.");
        }
    }
}

// checks if given player has won
fn check_for_win(array: &mut [[i32; 3]; 3], player: i32) {

    let help;

    match player {
        1 => help = String::from("[X]"),
        2 => help = String::from("[O]"),
        _ => help = String::new(),
    }

    // Check horizontal wins
    for hor in 0..3 {
        if array[hor][0] == player && array[hor][1] == player && array[hor][2] == player {
            display_game(array);
            println!("");
            println!("Player {} {help} wins!", player);
            process::exit(0);
        }
    }

    // Check vertical wins
    for vert in 0..3 {
        if array[0][vert] == player && array[1][vert] == player && array[2][vert] == player {
            display_game(array);
            println!("");
            println!("Player {} {help} wins!", player);
            process::exit(0);
        }
    }

    // Check diagonal wins
    if (array[0][0] == player && array[1][1] == player && array[2][2] == player)
        || (array[0][2] == player && array[1][1] == player && array[2][0] == player)
    {
        display_game(array);
        println!("");
        println!("Player {} {help} wins!", player);
        process::exit(0);
    }
}
