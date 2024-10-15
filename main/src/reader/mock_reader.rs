use std::{thread::sleep, time::Duration};

use tauri::{
    async_runtime::{spawn, JoinHandle},
    AppHandle, Manager,
};

use crate::{tags::Tag, GlobalState};

use super::{ReaderProtocol, REFRESH_INTERVAL};

/// Create a MockReader
///
/// This reader will sent a set of random tags
#[derive(Debug)]
pub struct MockReader {
    handle: Option<JoinHandle<()>>,
}

impl ReaderProtocol for MockReader {
    fn new(_hostname: String) -> Result<Self, super::ReaderError>
    where
        Self: Sized,
    {
        Ok(MockReader { handle: None })
    }

    fn start_reading<R: tauri::Runtime>(
        &mut self,
        app_handle: AppHandle<R>,
    ) -> Result<(), super::ReaderError> {
        let handle = spawn(async move {
            let tags_map = app_handle.state::<GlobalState>().tags_map.clone();
            let sleep_duration = Duration::from_millis(REFRESH_INTERVAL.into());
            loop {
                let tags = vec![
                    Tag::random(),
                    Tag::random(),
                    Tag::random(),
                    Tag::random(),
                    Tag::random(),
                    Tag::random(),
                    Tag::random(),
                    Tag::random(),
                    Tag::random(),
                    Tag::random(),
                ];
                tags_map.lock().unwrap().add_tags(tags);
                app_handle
                    .emit_all("updated-tags", tags_map.lock().unwrap().clone())
                    .unwrap();
                sleep(sleep_duration)
            }
        });
        self.handle = Some(handle);
        Ok(())
    }

    fn stop_reading(&mut self, _await_confirmation: bool) -> Result<(), super::ReaderError> {
        if let Some(handle) = self.handle.take() {
            handle.abort();
        };
        Ok(())
    }
}
