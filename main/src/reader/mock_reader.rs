use std::{thread::sleep, time::Duration};

use tauri::{
    async_runtime::{spawn, JoinHandle},
    AppHandle, Manager,
};

use crate::tags::{Tag, TagsMap};

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
            let sleep_duration = Duration::from_millis(REFRESH_INTERVAL.into());
            loop {
                let mut tags = vec![
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
                let new_map = TagsMap::from(tags.drain(..));
                app_handle.emit_all("updated-tags", new_map).unwrap();
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
