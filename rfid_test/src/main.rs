use std::process::{Command, Stdio};
use std::io::{self, BufReader, BufRead, Write};
use std::thread;
use std::sync::mpsc;

fn main() {
    // Path to the JAR file
    let jar_path = "src/apis/rfid_api_simple.jar";

    // Check if the JAR file exists
    if !std::path::Path::new(&jar_path).exists() {
        eprintln!("Error: JAR file not found at {}", jar_path);
        return;
    }

    // Spawn a child process to run the JAR file
    let mut child = match Command::new("java")
        .arg("-jar")
        .arg(jar_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(child) => child,
        Err(err) => {
            eprintln!("Error: Failed to run the JAR file: {}", err);
            return;
        }
    };

    // Extract stdout and stderr from the child process
    let stdout = child.stdout.take().expect("Failed to open stdout");
    let stdin = child.stdin.take().expect("Failed to open stdin");

    // Channel to send output from child process to main thread
    let (tx, rx) = mpsc::channel::<String>();

    // Spawn a thread to read and print the output from the JAR file
    let child_handle = thread::spawn(move || {
        let stdout_reader = BufReader::new(stdout);
        for line in stdout_reader.lines() {
            match line {
                Ok(line) => {
                    // Send output to main thread
                    if let Err(_) = tx.send(line) {
                        break; // If sending fails, break the loop
                    }
                }
                Err(err) => {
                    eprintln!("Error: Failed to read line from stdout: {}", err);
                    break; // Exit the loop on error
                }
            }
        }
    });

    // Spawn a thread to handle user input and pass it to the child process
    let input_handle = thread::spawn(move || {
        let mut stdin_writer = io::BufWriter::new(stdin);
        let mut input_data = String::new();
        loop {
            print!("Enter your input: ");
            io::stdout().flush().expect("Failed to flush stdout");
            if let Err(err) = io::stdin().read_line(&mut input_data) {
                eprintln!("Error: Failed to read input: {}", err);
                return;
            }

            // Write user input to child process stdin
            if let Err(err) = stdin_writer.write_all(input_data.as_bytes()) {
                eprintln!("Error: Failed to write to stdin: {}", err);
                return;
            }
            if let Err(err) = stdin_writer.flush() {
                eprintln!("Error: Failed to flush stdin: {}", err);
                return;
            }

            input_data.clear(); // Clear input buffer
        }
    });

    // Main thread reads output from child process
    for line in rx {
        println!("{}", line);
    }

    // Wait for the child process to exit
    let _ = child.wait();

    // Join the child thread to ensure it finishes before exiting
    let _ = child_handle.join();

    // Join the input thread to ensure it finishes before exiting
    let _ = input_handle.join();
}
