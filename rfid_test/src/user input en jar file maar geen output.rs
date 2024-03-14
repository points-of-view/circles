use std::process::{Command, Stdio};
use std::io::{self, Write, Read};

fn main() {
    // Path to the JAR file
    let jar_path = "src/apis/rfid_api.jar";

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

    // Get handles to child process stdin, stdout, and stderr
    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    let stdout = child.stdout.as_mut().expect("Failed to open stdout");
    let stderr = child.stderr.as_mut().expect("Failed to open stderr");

    // Read input from the user
    println!("Enter your input:");
    let mut input_data = String::new();
    if let Err(err) = io::stdin().read_line(&mut input_data) {
        eprintln!("Error: Failed to read input: {}", err);
        return;
    }

    // Write input to the child process
    if let Err(err) = stdin.write_all(input_data.as_bytes()) {
        eprintln!("Error: Failed to write to stdin: {}", err);
        return;
    }

    // Read output from the child process
    let mut output_data = String::new();
    if let Err(err) = stdout.read_to_string(&mut output_data) {
        eprintln!("Error: Failed to read from stdout: {}", err);
        return;
    }

    // Print the output of the command
    println!("Output: {}", output_data);

    // Handle errors from stderr if needed
    let mut error_data = String::new();
    if let Err(err) = stderr.read_to_string(&mut error_data) {
        eprintln!("Error: Failed to read from stderr: {}", err);
        return;
    }
    if !error_data.is_empty() {
        eprintln!("Error: {}", error_data);
    }

    // Wait for the child process to exit
    let _ = child.wait();
}
