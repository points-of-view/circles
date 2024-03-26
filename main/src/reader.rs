use std::{collections::HashMap, env};
use tauri::{
    api::process::{Command, CommandChild, CommandEvent}, async_runtime::{channel, Receiver}, AppHandle, Manager
};

const MAIN_JAVA_CLASS: &str = "reader.PrintRFIDReader.PrintRFIDTags";

// NOTE: We should create a mock method for this, so that development can work without a physical reader
pub fn spawn_reader(
    app: &AppHandle,
) -> (Receiver<CommandEvent>, CommandChild) {
    let resource_path = app
        .path_resolver()
        .resource_dir()
        .expect("Error while getting `resource_dir`");
    let resource_dir = resource_path.to_str().unwrap();

    if env::var("MOCK_RFID_READER").is_ok() {
        return spawn_mock_command();
    }

    let command = if cfg!(all(target_os = "linux", target_arch = "x86_64")) {
        linux_command(resource_dir)
    } else if cfg!(target_os = "windows") {
        windows_command(resource_dir)
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
    let (_rx, child) = Command::new("echo").args(["MOCK READER"]).spawn().unwrap();
    // NOTE: We create are own channel, so that we don't receive the actual output from the command we just spawned.
    // Instead we can use this channel to send some generated data
    // TODO: Our mock implementation is not yet sending any messages. We should generate some random responses
    let (_tx, rx) = channel(1);
    
    (rx, child)
}

fn windows_command(resource_dir: &str) -> Command {
    let vendor_jar = r"\_up_\reader\dist\vendor\zebra\lib\Symbol.RFID.API3.jar";
    let reader_jar = r"\_up_\reader\dist\reader.jar";

    Command::new("java").args([
        "-cp",
        &format!(
            "{path}{vendor_jar};{path}{reader_jar}",
            path = resource_dir,
            vendor_jar = vendor_jar,
            reader_jar = reader_jar
        ),
        MAIN_JAVA_CLASS,
    ])
}

fn linux_command(resource_dir: &str) -> Command {
    let library_path = "/_up_/reader/dist/vendor/zebra/lib";
    let vendor_path = format!(
        "{path}{lib_path}/x86_64",
        path = resource_dir,
        lib_path = library_path
    );
    let envs: HashMap<String, String> =
        HashMap::from([("LD_LIBRARY_PATH".into(), vendor_path.clone())]);

    Command::new("java").envs(envs).args([
        &format!("-Djava.library.path='{}'", vendor_path),
        "-cp",
        &format!(
            "{path}{lib_path}{vendor_jar}:{path}{jar_path}",
            path = resource_dir,
            lib_path = library_path,
            vendor_jar = "/Symbol.RFID.API3.jar",
            jar_path = "/_up_/reader/dist/reader.jar"
        ),
        MAIN_JAVA_CLASS,
    ])
}
