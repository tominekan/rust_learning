// src/cursed_choose_your_own_adventure.rs
use termion::color;
use std::io;
use std::process;

// Question struct
// has question, option_one, option_two
// they are all of the String type
struct Question {
    question: String,
    option_one: String,
    option_two: String
}

fn main() {
    let current_question = Question {
        question: String::from("You wake up and find yourself in a dungeon. What do you do?"),
        option_one: String::from("Try running out"),
        option_two: String::from("Explore the dungeon"),
    };

    let decision = get_input(current_question);

    if decision == 1 {
        oops("There was an ogre in front of the door and you got eaten.");
    }

    let current_question = Question {
        question: String::from("You walk around for a while an encounter a chest. Should you open it?"),
        option_one: String::from("Yes"),
        option_two: String::from("No"),
    };

    let decision = get_input(current_question);

    if decision == 1 {
        oops("You opened the chest to find a poisonous snake, that snake bit you, you died.");
    }

    let current_question = Question {
        question: String::from("You move on, as you walk, you notice heavy footsteps behind you, it is an ogre. You ..."),
        option_one: String::from("Investigate the ogre as you have never seen one before"),
        option_two: String::from("Run for your life, because ogres sounds scary"),
    };

    let decision = get_input(current_question);

    if decision == 2 {
        oops("As you run away, the ogre smells your fear and gobbles you up");
    }

    let current_question = Question {
        question: String::from("The ogre notices you are not afraid and feels threatened, so he walks towards you angrily. Are you going to..."),
        option_one: String::from("Intimidate it"),
        option_two: String::from("Reason with it"),
    };

    let decision = get_input(current_question);

    if decision == 2 {
        oops("You have gruesomely discovered that ogres are not reasonable creatures.");
    }

    let current_question = Question {
        question: String::from("The ogre is much bigger than you, you notice a banana on the floor. What do you do?"),
        option_one: String::from("Peel the banana and put it on the floor for the ogre to trip one"),
        option_two: String::from("Eat the banana to get +10 strength"),
    };

    let decision = get_input(current_question);

    if decision == 2 {
        oops("This is not a fantasy game, the ogre eats you and die, without ever gaining the +10 strength");
    }

    let current_question = Question {
        question: String::from("The ogre trips, falls on the floor, and dies a sad death. Triumphant, you decide that you have had enough dungeon adventure for one day. You head out, however, you notice a group of vampire bats, what do you do?"),
        option_one: String::from("Walk backwards slowly"),
        option_two: String::from("Continue on your path outwards"),
    };

    let decision = get_input(current_question);
    
    if decision == 1 {
        oops("The vampires notice you backing away, you get your blood sucked and die a painful death");
    }

    let current_question = Question {
        question: String::from("Luckily, the vampires did not notice you, and you exit safely. You decide that you ..."),
        option_one: String::from("Had a great time playing this game and want to do it again"),
        option_two: String::from("Hated the experience and wished you had never wasted your time playing this game"),
    };

    let decision = get_input(current_question);

    if decision == 2 {
        oops("The dead ogre came back to life and teamped up with the vampires bats to destroy you, you had no chance, and you died.");
    }

    println!("{}You enjoy a long successful life and have a large happy family. The end.", color::Fg(color::Green));
}

// Collect the input
fn get_input(question: Question) -> u8 {
    // Go in a loop forever unless the answer is correct
    loop {
        // Print all the questions
        println!("{}{}", color::Fg(color::Yellow), question.question);
        println!("    {}1. {}", color::Fg(color::Blue), question.option_one);
        println!("    {}2. {}", color::Fg(color::Magenta), question.option_two);
        println!("{}", color::Fg(color::Reset));

        // Collect the anwser to the question
        // Check if it is a valid answer (else move up the loop)
        // Else, move on and print a response to the answer
        let mut response = String::new();

        io::stdin()
            .read_line(&mut response)
            .expect("Could no stdin input for some reason");

        // Is an actual number
        let response: u8 = match response.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{} is not a number", response);
                continue;
            },
        };

        if !(response == 1) && !(response == 2) {
            println!("{} is not a number that is one or two", response);
            continue;
        }

        // Return the gotten response
        return response;
    }

}

// Just prints the text in red
fn oops(mistake: &str) {
    println!("{}{}", color::Fg(color::Red), mistake);
    process::exit(1)
}