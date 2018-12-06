use syntax::ast;

macro_rules! c_str {
    ($s:expr) => (
        concat!($s, "\0").as_ptr() as *const i8
    );
}

pub fn codegen(block: ast::Block, out_name: &str) {
    // TODO
}