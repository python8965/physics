use eframe::epaint::mutex::Mutex;

use std::mem;

use std::sync::Arc;
use tracing::info;

const BASE_URL: &str = "https://python8965.github.io/physics";

pub fn get_file(name: impl Into<String>, buffer: Arc<Mutex<Vec<u8>>>) {
    let request = ehttp::Request::get(format!("{BASE_URL}/{}", name.into()));

    ehttp::fetch(request, move |result| {
        let result = result.unwrap();
        let locked = &mut *buffer.lock();
        let _ = mem::replace(locked, result.bytes);
    });
}
