#![allow(unused)]  // used to ignore a compiler warning

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::io;

fn main() {
    // 
    println!("What is your name?");  // "!" = this function is a macro
    // every var declaration starts with "let"
    let mut name: String = String::new();  // instantiates a String
    let greeting: &str = "Nice to meet you"; // another way to init a string

    /* Interacting with User Input */
    // we make a call to collect a ref to a string (so that read_line() will save values directly to the "name" param)
    io::stdin().read_line(&mut name)  // returns a result enum - Ok, or Error
        .expect("Didn't Receive Input");   // error message w/ built-in exception handling

    println!("Hello {}! {}", name.trim_end(), greeting);
}
