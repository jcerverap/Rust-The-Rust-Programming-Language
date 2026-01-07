
use rand::Rng;

const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

pub fn do_variables(){

    println!("Guess the number!");

    let secret_number:u8 = rand::thread_rng().gen_range(1..=100);

    loop {
        
        // Variables exercise code
        println!("Please input your guess (x == 'exit').");
        
        let mut guess = String::new();
        
        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        if guess.trim() == "x" {
            println!("Goodbye.");
            break;

        } else {


            match guess.trim().parse::<u8>().unwrap().cmp(&secret_number) {
                    std::cmp::Ordering::Less => println!("Too small!"),
                    std::cmp::Ordering::Greater => println!("Too big!"),
                    std::cmp::Ordering::Equal => println!("You win!"),

            }
        }

    }

    second_task();
    do_shadowing();
    check_constants();

    return;
}

fn second_task(){

    println!("Second task:");

    let x:i8 = 5;
    let y:i8 = 10;

    println!("x = {x} and y + 2 = {}", y + 2);
    
}

fn do_shadowing(){

    println!("Shadowing:");

    let x:i8 = 0;
    println!("x = {x}");

    let x:i8 = x + 1;
    println!("x = {x}");

    {

        let x:i8 = x * 2;
        println!("x = {x}");
    }

    let spaces = "   ";
    let spaces = spaces.len();

    println!("Number of spaces: {spaces}");
}

fn check_constants(){

    println!("There are {THREE_HOURS_IN_SECONDS} seconds in three hours.");

}