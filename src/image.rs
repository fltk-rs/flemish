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
}
