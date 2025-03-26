mod ping;
mod chat;
mod identify;
// mod request_response;
mod distributed_key_value;
mod distributed_key_file;

use std::io;
fn main()
{
    println!("Enter \n1.Ping\n2.Identify\n3.Chat\n4.Distributed Key Value\n5.Distributed Key File");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    match input.trim().parse::<i32>().unwrap() {
        1 => ping::main().unwrap(),
        2 => identify::main().unwrap(),
        3 => chat::main().unwrap(),
        4 => distributed_key_value::main().unwrap(),
        5 => distributed_key_file::main().unwrap(),
        _ => println!("Invalid input"),
    }
}
