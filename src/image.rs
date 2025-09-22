use fltk::{prelude::*, *};
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::LazyLock;
use std::sync::Mutex;

pub(crate) static IMAGES: LazyLock<Mutex<HashMap<usize, Box<dyn ImageExt + Send>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

static NEXT_IDX: AtomicUsize = AtomicUsize::new(1);

#[derive(Default, Clone, Debug, PartialEq)]
pub struct Image {
    pub(crate) idx: usize,
}

impl Image {
    pub fn load<P: AsRef<str>>(path: P) -> Result<Self, FltkError> {
        let img = image::SharedImage::load(path.as_ref())?;
        let idx = NEXT_IDX.fetch_add(1, Ordering::Relaxed);
        IMAGES.lock().unwrap().insert(idx, Box::new(img));
        Ok(Self { idx })
    }

    pub fn scale(&self, width: i32, height: i32, proportional: bool, can_expand: bool) {
        if let Some(img) = IMAGES.lock().unwrap().get_mut(&self.idx) {
            img.scale(width, height, proportional, can_expand);
        }
    }

    pub fn copy_sized(&self, width: i32, height: i32) -> Option<Self> {
        // Create a deep copy resized to width x height, store it, and return a new handle
        let mut guard = IMAGES.lock().unwrap();
        if let Some(img) = guard.get(&self.idx) {
            if let Ok(rgb) = img.to_rgb_image() {
                let new_img = rgb.copy_sized(width, height);
                let idx = NEXT_IDX.fetch_add(1, Ordering::Relaxed);
                guard.insert(idx, Box::new(new_img));
                Some(Self { idx })
            } else {
                None
            }
        } else {
            None
        }
    }
}
