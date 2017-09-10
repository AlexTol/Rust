extern crate rand;

use std::io; //get input/output library, we are getting this library from 'prelude'
                //for additional dependencies/packages we must add 'crates' to the .toml
use rand::Rng; //rand comes from the crate we specified in the .toml file
                //'Rng' is a 'trait' that defines the methods that will be used to generate random numbers
use std::cmp::Ordering; //'cmp' is the method for comparing, 'Ordering' is an enum
                        //variant are Less, Greater, and Equal
fn main() {
    println!("Guess the number! (Please input your guess)");
    let secret_number = rand::thread_rng().gen_range(1, 101); //'thread_rng()' gives us a random function
                                                              //'gen_range()' let's us set a range
loop{
    //below we created a new instance of a 'String'
    let mut guess = String::new(); //'let' is used to create variables,
                                     //'mut' means this variable can be changed, variables are immutable by default
                                     //String::new() means we're making a new String
                                     //'::new()' tells us that 'new()' is an --associated function-- of 'String'
                                     //an associated function (static function) is implemented on a type

    io::stdin().read_line(&mut guess) //'stdin()' is an associated function of 'io'
        .expect("Failed to read line");  //we specify '&mut' to tell Rust that we are passing by reference
                                        //and that the variable is mutable
                                        //read_line() returns an io::Result, 'Result' types are enumerations
                                        //for result the types are 'Ok' or "Err"
                                            //Ok tells us the operation was good, and holds the generated values
                                            //Err tells us the operation failed, and holds error info
                                        //an insance of io::Result has the 'expect()' methods
                                            //depending on the type of 'Result' '.expect()' will behave differently
                                            //on 'Err' expect() will crash the program and display the message
                                            //on 'Ok' expect will just take the return value of 'Ok' and pass it to 'guess'

    let guess: u32 = match guess.trim().parse(){//the ':' after guess tells Rust we'll annotate the variable's type   ,.parse() turns guess into a number
    Ok(num) => num,
    Err(_) => continue,}; //so the program doesn't crash on invalid input



        println!("You guessed: {}",guess); //works like printf from C

        match guess.cmp(&secret_number) { //depending on which variant of 'Ordering' is returned, we go through with an action
        Ordering::Less    => println!("Too small!"), //this is facilitated by the 'match statement (kind of works like switch)
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal   => { //break out of loop on success
            println!("You win!");
            break;
        }
    }
}

}
