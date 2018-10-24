use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Symbol(u32);

impl Symbol {
    /// Maps a string to its interned representation.
    pub fn intern(string: &str) -> Self {
        with_interner(|interner| interner.intern(string))
    }

    pub fn interned(self) -> Self {
        with_interner(|interner| interner.interned(self))
    }

    pub fn gensym(string: &str) -> Self {
        with_interner(|interner| interner.gensym(string))
    }

    pub fn gensymed(self) -> Self {
        with_interner(|interner| interner.gensymed(self))
    }

    pub fn get(self) -> String {
        with_interner(|interner| String::from(interner.get(self)))
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.get())
    }
}

impl fmt::Debug for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Symbol({}, \"{}\")", self.0, self.get())
    }
}

#[derive(Default)]
pub struct Interner {
    names: HashMap<Rc<str>, Symbol>,
    strings: Vec<Rc<str>>,
    gensyms: Vec<Symbol>,
}

unsafe impl Send for Interner {}

impl Interner {
    pub fn new() -> Self {
        Interner::default()
    }

    pub fn intern(&mut self, string: &str) -> Symbol {
        if let Some(&name) = self.names.get(string) {
            return name;
        }

        let name = Symbol(self.strings.len() as u32);
        let string: Rc<str> = Rc::from(string);
        self.strings.push(string.clone());
        self.names.insert(string, name);
        name
    }

    pub fn interned(&self, symbol: Symbol) -> Symbol {
        if (symbol.0 as usize) < self.strings.len() {
            symbol
        } else {
            self.interned(self.gensyms[(!0 - symbol.0) as usize])
        }
    }

    fn gensym(&mut self, string: &str) -> Symbol {
        let symbol = self.intern(string);
        self.gensymed(symbol)
    }

    fn gensymed(&mut self, symbol: Symbol) -> Symbol {
        self.gensyms.push(symbol);
        Symbol(!0 - self.gensyms.len() as u32 + 1)
    }

    fn is_gensymed(&self, symbol: Symbol) -> bool {
        symbol.0 as usize >= self.strings.len()
    }

    pub fn get(&self, symbol: Symbol) -> &str {
        match self.strings.get(symbol.0 as usize) {
            Some(ref string) => string,
            None => self.get(self.gensyms[(!0 - symbol.0) as usize]),
        }
    }
}

#[inline]
fn with_interner<T, F: FnOnce(&mut Interner) -> T>(f: F) -> T {
    use lexer::globals::GLOBALS;
    f(&mut GLOBALS.symbol_interner.lock().unwrap())
}
