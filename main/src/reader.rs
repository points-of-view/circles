use std::collections::HashMap;
use tauri::{
    api::process::{Command, CommandChild, CommandEvent},
    AppHandle,
};

const JAR_PATH: &str = "/_up_/reader/dist/reader.jar";
const VENDOR_JAR: &str = "/Symbol.RFID.API3.jar";
const LIBRARY_PATH: &str = "/_up_/reader/dist/vendor/zebra/lib";
const MAIN_JAVA_CLASS: &str = "reader.PrintRFIDReader.PrintRFIDTags";

// NOTE: We should create a mock method for this, so that development can work without a physical reader
pub fn spawn_reader(
    app: AppHandle,
) -> (tauri::async_runtime::Receiver<CommandEvent>, CommandChild) {
    let resource_path = app
        .path_resolver()
        .resource_dir()
        .expect("Error while getting `resource_dir`");
    let resource_dir = resource_path.to_str().unwrap();

    if env::var("MOCK_RFID_READER").is_ok() {
        return spawn_mock_command();
    }

    command.spawn().unwrap()
}

fn spawn_mock_command() -> (Receiver<CommandEvent>, CommandChild) {
    let (_rx, child) = Command::new("echo").args(["MOCK READER"]).spawn().unwrap();
    // NOTE: We create are own channel, so that we don't receive the actual output from the command we just spawned.
    // Instead we can use this channel to send some generated data
    // TODO: Our mock implementation is not yet sending any messages. We should generate some random responses
    let (_tx, rx) = channel(1);
    
    (rx, child)
}

fn windows_command(resource_dir: &str) -> Command {
    Command::new("java").args([
        "-cp",
        &format!(
            "{path}{lib_path}{vendor_jar};{path}{jar_path}",
            path = resource_dir,
            lib_path = LIBRARY_PATH,
            vendor_jar = VENDOR_JAR,
            jar_path = JAR_PATH
        ),
        MAIN_JAVA_CLASS,
    ])
}

#[cfg(not(target_os = "windows"))]
fn unix_command(resource_dir: &str) -> Command {
    let vendor_path = format!(
        "{path}{lib_path}/x86_64",
        path = resource_dir,
        lib_path = LIBRARY_PATH
    );
    let envs: HashMap<String, String> =
        HashMap::from([("LD_LIBRARY_PATH".into(), vendor_path.clone())]);

    Command::new("java").envs(envs).args([
        &format!("-Djava.library.path='{}'", vendor_path),
        "-cp",
        &format!(
            "{path}{lib_path}{vendor_jar}:{path}{jar_path}",
            path = resource_dir,
            lib_path = LIBRARY_PATH,
            vendor_jar = VENDOR_JAR,
            jar_path = JAR_PATH
        ),
        MAIN_JAVA_CLASS,
    ])
}
