use std::io;
use rand::prelude::*;
use std::fs;

fn main() {

    println!("Hey! Input the length of you Password:");

    let mut length = String::new();

    io::stdin().read_line(&mut length).expect("Please input your password length: ");
    

    let p_length: i32 = length.trim().parse().expect("Input your a valid number: ");


    let mut generatedp = generate(p_length);

    let filename = "passwords.txt";

    match fs::write(filename, &generatedp) {
        Ok(()) => println!("Your file has been saved to : {:?}", filename),
        Err(e) => println!("Error saving your password: {:?}", e)
    }


    println!("Your generated password is : {}", &generatedp);
}

fn generate(r: i32) -> String {
    let mut main = Vec::new();

    let alpha_lowercase: Vec<char> = ('a'..'z').collect();
    let alpha_uppercase: Vec<char> = ('A'..'Z').collect();
    let symbols: Vec<char> = "!@#$%^&*()_+-=[]{}".chars().collect();


    loop {
        for l in alpha_lowercase {
            main.push(l);
        } 
        for u in alpha_uppercase {
            main.push(u);
        }
        for s in symbols {
            main.push(s);
        }
        break
    }

    let mut word = String::new();
    let mut yours = 0; 


    let mut seed = rand::rng();
    

    loop {

        let mut range = seed.random_range(0..main.len());

        if yours < r {
            word.push(main[range]);
            yours += 1;
        } else {
            break
        }
         
    }
    
    word


}
