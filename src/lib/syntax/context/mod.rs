use fnv::FnvHashMap;
use id_arena::Arena;

use syntax::source_map::SourceMap;
use syntax::token::Symbol;

pub struct Context {
    strings: Arena<String>,
    interned: FnvHashMap<String, Symbol>,
    source_map: SourceMap,
}

impl Context {
    pub fn new() -> Self {
        Context {
            strings: Arena::new(),
            interned: FnvHashMap::default(),
            source_map: SourceMap::new(),
        }
    }

    pub fn intern<S: AsRef<str> + Into<String>>(&mut self, s: S) -> Symbol {
        if let Some(id) = self.interned.get(s.as_ref()) {
            return *id;
        }

        let s = s.into();
        let id = self.strings.alloc(s.clone());
        self.interned.insert(s, id);
        id
    }

    pub fn get_string(&self, symbol: Symbol) -> &str {
        &self.strings[symbol]
    }

    pub fn get_source_map(&mut self) -> &mut SourceMap {
        &mut self.source_map
    }
}
