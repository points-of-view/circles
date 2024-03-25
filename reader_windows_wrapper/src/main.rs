use std::{
    env,
    process::{self, Command},
};

fn main() {
    let mut args = env::args();
    // args[0] is file of executable
    args.next();
    let path = match args.next() {
        Some(arg) => arg,
        None => {
            eprintln!("You should pass a folder to this script");
            process::exit(1)
        }
    };
    println!("path: {}", path);
    Command::new("java").args([
        "-cp",
        &format!("{path}/_up_/reader/dist/vendor/zebra/lib/Symbol.RFID.API3.jar;{path}/_up_/reader/dist/reader.jar", path = path),
        "reader.PrintRFIDReader.PrintRFIDTags",
    ]).status().expect("Java failed to start");
}
