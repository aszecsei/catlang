use inkwell::support::LLVMString;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CodeGenError {
    #[error("LLVM error")]
    LLVMError(String),
    #[error("could not create target machine")]
    CouldNotCreateTargetMachine,
    #[error("unknown error")]
    Unknown,
}

impl From<LLVMString> for CodeGenError {
    fn from(llvm_str: LLVMString) -> Self {
        let llvm_string = llvm_str.to_string();
        CodeGenError::LLVMError(llvm_string)
    }
}
