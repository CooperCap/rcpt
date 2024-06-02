use curl::easy;
use std::{io::Write,  fs::File};

fn main() {
    let mut curling = easy::Easy::new();
    let mut store_file = match File::open("config.temp") {
        Ok(file) => file,
        Err(Errormesg) => File::create("config.temp").unwrap()
    };

    store_file.write(b"Hello bro").unwrap();
    match curling.url("https://www.google.com") {
        Ok(_) => print!("Success!\n"),
        Err(_) => print!("Problem bro, \n")
    }
    match curling.perform() {
        Ok(_) => println!("Performance all ok!"),
        Err(_) => println!("PWP")
    };
    let thing = chrono::Local::now();
    //My first comment
    println!("Hello, world!, The time rn is {:?}", thing.time());

    println!("Hello world");
}
