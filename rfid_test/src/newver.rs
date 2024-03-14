use std::process::{Command, Stdio};
use std::io::{self, BufRead, Write};

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

    // Get handle to child process stdout
    let stdout = child.stdout.take().expect("Failed to open stdout");

    // Create a reader for stdout
    let stdout_reader = io::BufReader::new(stdout);

    // Iterate over lines from the JAR file output
    for line in stdout_reader.lines() {
        match line {
            Ok(line) => {
                println!("{}", line); // Print output from the JAR file
                let stdin_writer = child.stdin.as_mut().expect("Failed to open stdin");
                let mut input_data = String::new();
                // print!("Enter your input: ");
                io::stdout().flush().expect("Failed to flush stdout");
                if let Err(err) = io::stdin().read_line(&mut input_data) {
                    eprintln!("Error: Failed to read input: {}", err);
                    return;
                }
                if let Err(err) = stdin_writer.write_all(input_data.as_bytes()) {
                    eprintln!("Error: Failed to write to stdin: {}", err);
                    return;
                }
                if let Err(err) = stdin_writer.flush() {
                    eprintln!("Error: Failed to flush stdin: {}", err);
                    return;
                }
            }
            Err(err) => {
                eprintln!("Error: Failed to read line from stdout: {}", err);
                return;
            }
        }
    }

    // Wait for the child process to exit
    let _ = child.wait();
}
