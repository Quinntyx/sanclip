pub mod model;

use clipboard_master::Master;
use copypasta::ClipboardContext;

use crate::model::sanitizer::Sanitizer;

fn main() {
    let _ = Master::new(Sanitizer {
            clip: ClipboardContext::new().expect("Should be able to create clipboard context")
        }).run();
}

