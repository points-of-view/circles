use std::io::{BufRead, BufReader};
use std::process::{Child, Command, Stdio};
use std::sync::mpsc;
use std::thread;

pub fn run_instance() -> Result<Child, String> {
    let jar_path = "src/apis/rfid_api.jar";
    // Check if the JAR file exists
    if !std::path::Path::new(&jar_path).exists() {
        return Err(format!("Error: JAR file not found at {}", jar_path));
    }

    // Spawn a child process to run the JAR file
    let mut child = match Command::new("java")
        .arg("-jar")
        .arg(jar_path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(child) => child,
        Err(err) => {
            return Err(format!("Error: Failed to run the JAR file: {}", err));
        }
    };

    // Extract stdout and stderr from the child process
    let stdout = child.stdout.take().expect("Failed to open stdout");

    // Channel to send output from child process to main thread
    let (tx, rx) = mpsc::channel::<String>();

    // Spawn a thread to read and print the output from the JAR file
    thread::spawn(move || {
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

    // Main thread reads output from child process
    for line in rx {
        println!("{}", line);
    }
    Ok(child)
}
