use crate::syntax::ast;

macro_rules! c_str {
    ($s:expr) => {
        concat!($s, "\0").as_ptr() as *const i8
    };
}

pub fn codegen(_block: ast::Program, _out_name: &str) {
    // TODO
}
