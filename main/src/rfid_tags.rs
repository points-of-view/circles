use serde::Serialize;
use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::process::{Child, Command, Stdio};
use std::sync::{mpsc, Arc};
use std::thread;
use std::time::{Duration, Instant};

#[derive(Serialize, Clone, Debug)]
pub struct Tag {
    id: String,
    strength: i8,
    antenna: u8,
}

const JAR_PATH: &str = "src/apis/PrintRFIDTags.jar";

pub fn run_instance() -> Result<Child, String> {
    // Check if the JAR file exists
    if !std::path::Path::new(&JAR_PATH).exists() {
        return Err(format!("Error: JAR file not found at {}", JAR_PATH));
    }

    // Spawn a child process to run the JAR file
    let mut child = match Command::new("java")
        .arg("-jar")
        .arg(JAR_PATH)
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
    let (tx, _rx) = mpsc::channel::<HashMap<std::string::String, Tag>>();

    // HashMap to store the latest signal strength for each unique ID
    let tag_map: Arc<std::sync::Mutex<HashMap<String, Tag>>> =
        Arc::new(std::sync::Mutex::new(HashMap::new()));

    // Starting point to measure elapsed time
    let mut start = Instant::now();

    // Spawn a thread to read and print the output from the JAR file
    thread::spawn(move || {
        let stdout_reader = BufReader::new(stdout);
        for line in stdout_reader.lines() {
            if let Ok(line) = line {
                // Parse the line
                let parts: Vec<&str> = line.split('|').collect();
                if parts.len() == 3 {
                    let id = parts[0].to_string();
                    let antenna = parts[1].parse::<u8>().unwrap_or_default();
                    let strength = parts[2].parse::<i8>().unwrap_or_default();

                    // Update the hashmap with the latest signal strength
                    let mut tag_map = tag_map.lock().unwrap();
                    if start.elapsed() > Duration::from_millis(500) {
                        // Send the tags through the channel
                        if let Err(_) = tx.send(tag_map.clone()) {
                            break;
                        }
                        start = Instant::now();
                        tag_map.drain();
                    } else {
                        let tag = tag_map.entry(id.clone()).or_insert(Tag {
                            id: id.clone(),
                            strength: i8::MIN, // Initialize with minimum value
                            antenna: 0,        // Initialize with 0
                        });

                        if strength > tag.strength {
                            tag.strength = strength;
                            tag.antenna = antenna;
                        }
                    }
                }
            }
        }
    });

    // // Main thread reads output from child process
    // for tag in rx {
    //     println!("{:?}", tag);
    // }
    Ok(child)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_err_if_jar_is_not_found() {
        assert!(std::path::Path::new(&JAR_PATH).exists())
    }
}
