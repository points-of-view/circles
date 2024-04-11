use dunce::simplified;
use std::thread::sleep;
use std::time::{Duration, Instant};
use std::{collections::HashMap, env, path::PathBuf};
use tauri::{
    api::process::{Command, CommandChild, CommandEvent},
    async_runtime::{channel, spawn, Receiver},
};

use crate::tags::create_mock_tag;

const MAIN_JAVA_CLASS: &str = "reader.PrintRFIDReader.PrintRFIDTags";

// NOTE: We spawn a slightly different command for different OSes/architectures
// Since this code is platform specific, it is hard to test. Make sure you validate any changes on actual devices
pub fn spawn_reader(resource_path: PathBuf) -> (Receiver<CommandEvent>, CommandChild) {
    if env::var("MOCK_RFID_READER").is_ok() {
        return spawn_mock_command();
    }

    let command = if cfg!(all(target_os = "linux", target_arch = "x86_64")) {
        linux_command(resource_path)
    } else if cfg!(target_os = "windows") {
        windows_command(resource_path)
    } else {
        panic!(
            "`spawn_reader` can not be called on {arch}-{os}. The java SDK and underlying native libraries are only supported on windows and linux. Use `MOCK_RFID_READER=1` to mock the reader output",
            arch = env::consts::ARCH,
            os = env::consts::OS
        )
    };

    command.spawn().unwrap()
}

fn spawn_mock_command() -> (Receiver<CommandEvent>, CommandChild) {
    #[cfg(target_os = "windows")]
    let (_rx, child) = Command::new("cmd")
        .args(["echo MOCK READER"])
        .spawn()
        .unwrap();
    #[cfg(not(target_os = "windows"))]
    let (_rx, child) = Command::new("echo").args(["MOCK READER"]).spawn().unwrap();
    // NOTE: We create are own channel, so that we don't receive the actual output from the command we just spawned.
    // Instead we can use this channel to send some generated data
    let (tx, rx) = channel(1);

    // We send a random tag every 100MS
    let interval = Duration::from_millis(100);
    let mut next_time = Instant::now() + interval;
    spawn(async move {
        loop {
            let _ = tx.send(CommandEvent::Stdout(create_mock_tag())).await;
            let _ = tx
                .send(CommandEvent::Stderr(String::from("Some error occurs")))
                .await;
            sleep(next_time - Instant::now());
            next_time += interval;
        }
    });

    (rx, child)
}

fn windows_command(resource_path: PathBuf) -> Command {
    let vendor_jar = resource_path.join(r"_up_\reader\dist\vendor\zebra\lib\Symbol.RFID.API3.jar");
    let reader_jar = resource_path.join(r"_up_\reader\dist\reader.jar");

    // We need to simplify these paths, and remove any optional UNC prefix (`//?`), since java can't resolve this
    let simplified_vendor_jar = simplified(&vendor_jar.as_path());
    let simplified_reader_jar = simplified(&reader_jar.as_path());

    Command::new("java").args([
        "-cp",
        &format!(
            "{};{}",
            simplified_vendor_jar.to_string_lossy(),
            simplified_reader_jar.to_string_lossy()
        ),
        MAIN_JAVA_CLASS,
    ])
}

fn linux_command(resource_path: PathBuf) -> Command {
    let library_path = resource_path.join("_up_/reader/dist/vendor/zebra/lib/x86_64");
    let vendor_jar = resource_path.join("_up_/reader/dist/vendor/zebra/lib/Symbol.RFID.API3.jar");
    let reader_jar = resource_path.join("_up_/reader/dist/reader.jar");

    let envs: HashMap<String, String> = HashMap::from([(
        "LD_LIBRARY_PATH".into(),
        library_path.to_string_lossy().into_owned(),
    )]);

    Command::new("java").envs(envs).args([
        &format!("-Djava.library.path='{}'", library_path.to_string_lossy()),
        "-cp",
        &format!(
            "{}:{}",
            vendor_jar.to_string_lossy(),
            reader_jar.to_string_lossy()
        ),
        MAIN_JAVA_CLASS,
    ])
}
