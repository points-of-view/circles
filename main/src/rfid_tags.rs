use std::process::Command;

fn main() {
    // Command to run the JAR file
    let output = Command::new("java")
        .arg("-jar")
        .arg("apis/rfid_api.jar") // Replace "your_jar_file.jar" with the actual path to your JAR file
        .output()
        .expect("Failed to run the JAR file");

    // Print the output of the command
    println!("Output: {:?}", output);
}