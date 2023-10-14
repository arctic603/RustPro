
use std::cmp::Ordering;
use std::io;
use rand::Rng;

pub fn guessfunc() {

    let  secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please type a number");
        let mut guess= String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        //trim() 去除 字符串开头和结尾的空白字符
        // parse() 去除 字符串转换成其他类型。且转换的类型需要let xx: type来指定
        let guess : u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal=> {println!("you win!");break}
        }

    }

}


/*
use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn guessfunc() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
*/