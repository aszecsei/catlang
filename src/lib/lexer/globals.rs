use lexer::symbol::Interner;
use std::sync::{Arc, Mutex};

#[derive(Default)]
pub struct Globals {
    pub symbol_interner: Arc<Mutex<Interner>>,
}

impl Globals {
    pub fn new() -> Globals {
        Globals {
            symbol_interner: Arc::from(Mutex::new(Interner::new())),
        }
    }
}

lazy_static! {
    pub static ref GLOBALS: Globals = Globals::new();
}
