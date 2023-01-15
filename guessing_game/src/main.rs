use rand::Rng;
use std::io; //bring the input/output library into scope from the standard library, like in C++
use std::cmp::Ordering;

//the set of items brought into scope by default is called the perlude,
//if the type isn't in the perlude, you bring it into scope with "use"

//entry point into the program
fn main() {
    println!("Guess the number!");
    loop{
    println!("Please input your guess.");

    //we use the let statement to create a variable
    //variables are immutable by default, so we use the "mut" keyword
    //to allow us to change the value of a varialbe
    
    //let apples = 5;  is immutable
    //let mut bananas = 5; is mutable
    
    //String::new() is a function that returns a new instance of String
    //the :: syntax indicated that new is associated to the String type
    let mut guess = String::new();

    let secret_number = rand::thread_rng().gen_range(1..=100);

    //println!("The secret number is : {secret_number}");

    //call stdin from the io that we've included earlier
    io::stdin()
        .read_line(&mut guess) //we parse &mut guess as the arguments to tell it the string to
                               //store input into, the argument needs to be mutable so we can
                               //modify it
                               //the & indicates that it's a reference, which gives a way for
                               //multiple parts of code to access that piece of data into memory
                               //wihtout copying it over. References are also immutable by default,
                               //which is why we write &mut guess rather than &guess.
        .expect("Failed to read line"); //the results are "Ok" and "Err", if read_line returns an
                                        //Err, expect will output that error, otherwise it will
                                        //take take the return value that Ok is holding and return
                                        //it to be ready for use
    //printing a placeholder variable in a line
    println!("You guessed: {guess}");

    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win!");
            break;
        },
    }
    }

}
