use eframe::epaint::mutex::Mutex;
use std::cell::RefCell;
use std::mem;
use std::rc::Rc;
use std::sync::Arc;

const BASE_URL: &str = "https://python8965.github.io/physics";

pub struct FileFetcher {}

pub fn get_file(name: impl Into<String>, buffer: Arc<Mutex<Vec<u8>>>) {
    let request = ehttp::Request::get(format!("{BASE_URL}/{}", name.into()));

    ehttp::fetch(request, move |result| {
        let result = result.unwrap();
        let locked = &mut *buffer.lock();
        let _ = mem::replace(locked, result.bytes);
    });
}
