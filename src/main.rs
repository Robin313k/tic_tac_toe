use clearscreen::clear;
use colored::*;
use crossterm::{
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use std::io::stdout;
use std::{io, process};

fn main() -> std::io::Result<()> {
    let mut stdout = stdout();

    // Enter the alternate screen
    stdout.execute(EnterAlternateScreen)?;

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
            display_game(&field, false);
            change_slot(&mut field, player_one);
            if check_for_win(&mut field, player_one) == true {
                break;
            }
            player_one_beginns = false;
        } else {
            display_game(&field, false);
            change_slot(&mut field, player_two);
            if check_for_win(&mut field, player_two) == true {
                break;
            }
            player_one_beginns = true;
        }
        check_for_win(&mut field, player_one);
        check_for_win(&mut field, player_two);
        // check if any slots are unclaimed, if so draw happens, bc no one won
        if check_for_zero(&field) == false {
            break;
        }
    }
    if check_for_zero(&field) == false {
        // show final result
        display_game(&field, true);
        println!("");
        println!("Draw!");
    }

    let mut wait = String::new();
    println!("Press enter to leave.");
    io::stdin()
        .read_line(&mut wait)
        .expect("Failed to read line");

    stdout.execute(LeaveAlternateScreen)?;

    Ok(())
}

// display gamefield in a more appealing way
fn display_game(array: &[[i32; 3]; 3], display_outcome: bool) {
    // vars for empty, player one and player two
    let x = String::from("X");
    let o = String::from("O");

    // clear screen so player wont need to scroll
    clear().expect("failed to clear screen");
    // starts process of printing gamefield
    let mut counter = 0;
    for i in 0..3 {
        for j in 0..3 {
            counter = counter + 1;
            match array[i][j] {
                0 => {
                    if display_outcome {
                        print!("[ ]")
                    } else {
                        print!("[{}]", counter)
                    }
                }
                1 => print!("[{}]", &x.red()),
                2 => print!("[{}]", &o.blue()),
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
    let help;

    match player {
        1 => help = String::from("X"),
        2 => help = String::from("O"),
        _ => help = String::new(),
    }

    let mut already_taken = false;

    // Loop to prevent false input from crashing the game
    loop {
        // shadow input every iteration so game does not crash
        let mut input = String::new();

        // Display the game field
        display_game(array, false);
        println!("");
        println!("Enter q to quit");
        if player == 1 {
            println!("Player {} [{}] is now playing!", player, help.red());
        } else {
            println!("Player {} [{}] is now playing!", player, help.blue());
        }

        // check if slot is claimed or not
        if already_taken {
            println!("Slot already taken, choose another one.");
        }

        // Prompt for input
        println!("Input the field you want to claim (1-9):");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // Code to quit game
        if input.trim() == "q" {
            println!("Are you sure you want to quit? [y/N]");

            let mut exit_question = String::new();
            io::stdin()
                .read_line(&mut exit_question)
                .expect("Failed to read line");

            //  second question to disable accidental quitting
            if exit_question.trim() == "Y" || exit_question.trim() == "y" {
                process::exit(0);
            }
        }

        // transfer input from String to unsigned 8 bit int
        let input: u8 = match input.trim().parse() {
            Ok(num) if num >= 1 && num <= 9 => num, // Only accept input between 1 and 9
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
            break; // Exit the loop when a valid move is made
        } else {
            already_taken = true;
        }
    }
}

// checks if given player has won
fn check_for_win(array: &mut [[i32; 3]; 3], player: i32) -> bool {
    let help;

    match player {
        1 => help = String::from("X"),
        2 => help = String::from("O"),
        _ => help = String::new(),
    }

    let mut someone_won = false;

    // Check horizontal wins
    for hor in 0..3 {
        if array[hor][0] == player && array[hor][1] == player && array[hor][2] == player {
            someone_won = true;
        }
    }

    // Check vertical wins
    for vert in 0..3 {
        if array[0][vert] == player && array[1][vert] == player && array[2][vert] == player {
            someone_won = true;
        }
    }

    // Check diagonal wins
    if (array[0][0] == player && array[1][1] == player && array[2][2] == player)
        || (array[0][2] == player && array[1][1] == player && array[2][0] == player)
    {
        someone_won = true;
    }

    if someone_won {
        display_game(array, true);
        println!("");
        if player == 1 {
            println!("Player {} [{}] wins!", player, help.red());
        } else {
            println!("Player {} [{}] wins!", player, help.blue());
        }
        return true;
    } else {
        return false;
    }
}
