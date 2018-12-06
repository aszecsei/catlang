use syntax::ast;

use llvm::core::*;
use llvm::bit_writer::*;

macro_rules! c_str {
    ($s:expr) => (
        concat!($s, "\0").as_ptr() as *const i8
    );
}

pub fn codegen(block: ast::Block, out_name: &str) {
    unsafe {
        let context = LLVMContextCreate();
        let module = LLVMModuleCreateWithName(c_str!("main"));
        let builder = LLVMCreateBuilderInContext(context);

        LLVMWriteBitcodeToFile(module, out_name);
        LLVMDisposeBuilder(builder);

        LLVMDisposeModule(module);
        LLVMContextDispose(context);
    }
}