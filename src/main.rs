use std::io;
use std::cmp::Ordering;
use rand::Rng;

struct Billy {
    name: String,
    happiness: i32,
    stomach: f32,
    obedient: bool,
}

struct CatFood {
    name: String,
    happiness_value: i32,
    stomach_value: f32
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {width: size, height: size}
    }
}

fn main() {
    let qbilly = Billy {
        name: String::from("Billy"),
        happiness: 50,
        stomach: 0.5,
        obedient: true
    };

    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    let square1 = Rectangle::square(10);
    
    println!("Check out this sweet square {:#?}", square1);
    println!("Can rect 1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect 1 hold rect3? {}", rect1.can_hold(&rect3));

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    std::process::exit(match run_app(billy) {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("error: {:?}", err);
            1
        }
    });
}
 
fn run_app(billy: Billy) -> Result<(), ()>{
    greeting();
    start_interaction(billy);
    Ok(())
}

fn greeting() {
    println!("|  reeeOWWW!              |");
    println!("|                         |");
    println!("|  How's it going today? I'm doing well, and I want to help.  |");
}

fn start_interaction(billy: Billy) {
    print_billy();
    println!("|  I'm always hungry if you'd like to *feed* me. We could also play a *game*!   |");
    println!("|  Hoping for something more useful? Check out my *tools*                       |");
    println!("|  I could also use a nice nap if you'd like to *quit* for now.                 |");
    println!("|  Please type on option to continue:                                           |");
    println!("|  *feed*    *game*    *tools*   *quit*   *status*                              |");
    loop {

        let mut response = String::new();

        io::stdin().read_line(&mut response)
            .expect("Rrrrr. I'm not sure what you mean...");
        
        match response.trim() {
            "feed" => {
                feeding(billy);
                break;
            },
            "game" => {
                games(billy);
                break;
            },
            "play" => {
                games(billy);
                break;
            },
            "tools" => {
                tools(billy);
                break;
            },
            "status" => {
                show_status(billy);
                break;
            },
            "quit" => {
                quit();
                break;
            },
            "q" => {
                quit();
                break;
            },
            _ => {
                undefined_choice(billy, response);
                break;
            }
        }
    }
}

// STATUS

fn show_status(billy: Billy){
    println!("Happiness: {}", billy.happiness);
    println!("Stomach {}% full", (billy.stomach * 100.0 ) );
    start_interaction(billy);
}

// FEEDING

fn feeding(mut billy: Billy) {
    println!("*You fed Billy his favorite meal*");
    println!("RrrOWWooWWW! My strength is returning! Soon I'm sure I'll be able to help you out");
    println!("");

    billy.happiness += 10;
    billy.stomach += 0.05;
    start_interaction(billy);
}

// GAMES
fn games(billy: Billy) {
    println!("I'm thinking of a number, can you guess what it is?");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    let mut num_guesses = 0;
    loop {

        println!("Enter a guess, please:");
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("I didn't get that...");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);
        num_guesses += 1;
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                win_guessing_game(num_guesses, secret_number);
                break;
            }
        }
    }

    println!("...");
    println!("...");
    println!("...");
    println!("");

    start_interaction(billy);

}

fn win_guessing_game(num_guesses: u32, secret_number: u32){
    // If the player got the answer in one guess post a special message
    if num_guesses == 1 {
        println!("Mee-WOWOWOWOW! YOU GOT IT IN ONE GUESS!!! Are you psychic? We're really in sync.");
        return;
    }

    if secret_number == 13 {
        println!("SpooOoOoOoky! The secret number was 13. Don't worry buddy, it's not bad luck. Good guess!");
        return;
    }
    
    // For any other number of guesses respond with a random message of congratulations.
    let congratulations = ["Meow, yes! You got it!",  "Yep! Mrrrrow you got it!", "Not bad, meow, That was it!", "Ding ding ding! Winner winner chicken dinner! Also... I'd like a chicken dinner", "That's it! Mee-WOW, we've got a winner over here!"];

    let response = rand::thread_rng().gen_range(1, 6);
    println!("{}", congratulations[response]);
    
}

// TOOLS

fn tools(billy: Billy) {
    println!("Reee-OW, welcome to my lab! Here are some helpful things I can do:");
    println!("1. Convert temperatures");

    let mut response = String::new();
    loop {
        io::stdin().read_line(&mut response)
                .expect("Rrrrr. I'm not sure what you mean...");


        match &response.trim() {
            &"1" => {
                convert_temperatures(billy);
                break;
            },
            _ => {
                undefined_choice(billy, response);
                break;
            }
        }
    }
}

fn convert_temperatures(billy: Billy){
    println!("Is your temperature in Fahrenheit or Celsius?");
    println!("1. Fahrenheit");
    println!("2. Celsius");
    let mut response = String::new();
    loop {
        io::stdin().read_line(&mut response)
                .expect("Rrrrr. I'm not sure what you mean...");


        match &response.trim() {
            &"1" => {
                fahrenheit_to_celsius();
                break;
            }
            &"2" => {
                celsius_to_fahrenheit();
                break;
            }
            _ => {
                return undefined_choice(billy, response);
            }
        }
    }
    tools(billy);
}

fn fahrenheit_to_celsius() {
    println!("Please enter your Fahrenheit temperature in the form of an integer:");

    let mut f_temp = String::new();
    io::stdin().read_line(&mut f_temp)
            .expect("Rrrrr. I'm not sure what you mean...");
    
    let f_temp: f32 = match f_temp.trim().parse() {
            Ok(num) => num,
            Err(_) => 0.0,
        };
    let c_temp: f32 = ((f_temp - 32.0) * 0.5556) as f32;
    println!("{} Fahrenheit = {} Celsius", f_temp, c_temp);
}

fn celsius_to_fahrenheit(){
    println!("Please enter your Celsius temperature in the form of an integer:");

    let mut c_temp = String::new();
    io::stdin().read_line(&mut c_temp)
            .expect("Rrrrr. I'm not sure what you mean...");
    
    let c_temp: f32 = match c_temp.trim().parse() {
            Ok(num) => num,
            Err(_) => 0.0,
        };
    let f_temp: f32 = ((c_temp * 1.8) + 32.0) as f32;
    println!("{} Celsius = {} Fahrenheit", c_temp, f_temp);
}

fn quit() {
    // End program
    println!("See you next time friendo!");
}

fn undefined_choice(billy: Billy, response: String) {
    println!("You chose to: {}", response);
    println!("Rrr... not sure what to do with that. Let's try something else");
    println!("");
    start_interaction(billy);
}

// GRAPHICS
fn print_billy() {
    println!("|                           |");
    println!("|        /\\  /\\             |");
    println!("|       = ^ . ^ =           |");
    println!("|       \\      /            |");
    println!("|   *  /      \\             |");
    println!("| ||  |        \\            |");
    println!("|  \\\\ |   \\\\   \\\\           |");
    println!("|   \\\\/____\\\\_ _\\\\          |");
    println!("|___________________________|");
    println!("");
}
