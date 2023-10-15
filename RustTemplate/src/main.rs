use std::io;
use crate::GuessingGame::guessfunc;
use rand::Rng;
mod GuessingGame;

fn variablefunc()
{
    let var = 3;

    let mut var = 4;    //let 会导致之前的变量被 Shadowing隐藏起来了。

    println!("{var}");

    // 常量声明
    const THREE_HOURS_IN_SECONDS: u32 = 60*60*3;


    // 一般是为了去改变类型，主要是为了满足实际需求
    let spaces = "     ";
    let spaces = spaces.len();
    // mut的特点
    let mut spaces = "      ";
    //spaces = spaces.len();类型无法改变

    let guess: u32 = "42".parse().expect("Not a number!");

    //boolean
    let t = true;
    let heart_eyed_cat = '😻';

    //元组类型
    //create
    let tup: (i32,f64,u8) = (50,63.2,5);

    let (x,y,z) = tup;
    let xval = tup.0;
    let yval = tup.1;
    let zval = tup.2;

    let months = ["January","February"];

    //
    let array:[i32;5] = [1,2,5,6,7];

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index.trim().parse().expect("Index entered was not anumber");
    let element = array[index];

    println!("The value of the element at index {index} is: {element}");

    //expression
    let express = {
        let x = 5;
        x + 32
    };


}

fn getval(x: i32)->i32 {
    let secret_number = rand::thread_rng().gen_range(1..=100);
   // x = x + 5;
    if secret_number > 50 && secret_number < 100 {
        println!("secret_number > 50 && secret_number < 100");
    }
    secret_number
}

fn main() {
    // use GuessingGame::guessfunc;
    // guessfunc();

    //variablefunc();
    another_function(3);
    let mut number = 10;
    let random_val = getval(number);
    println!("{}...{}",random_val,number);
}


fn another_function(x:i32) {
    println!("The value of x is : {x}");
}