use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::targets::{CodeModel, FileType, RelocMode, Target, TargetMachine, TargetTriple};
use inkwell::OptimizationLevel;
use lazy_static::lazy_static;
use std::path::Path;

mod error;

lazy_static! {
    pub static ref DEFAULT_TARGET_TRIPLE: String = TargetMachine::get_default_triple()
        .as_str()
        .to_string_lossy()
        .into_owned();
}

pub struct CodeGen<'ctx> {
    pub context: &'ctx Context,
    pub module: Module<'ctx>,
    pub builder: Builder<'ctx>,
}

impl<'ctx> CodeGen<'ctx> {
    pub fn new(context: &'ctx Context) -> Self {
        let module = context.create_module("example");
        let builder = context.create_builder();

        CodeGen {
            context,
            module,
            builder,
        }
    }

    pub fn compile_example(&mut self) -> Option<()> {
        let i64_type = self.context.i64_type();
        let fn_type = i64_type.fn_type(&[i64_type.into(), i64_type.into(), i64_type.into()], false);
        let function = self.module.add_function("sum", fn_type, None);
        let basic_block = self.context.append_basic_block(function, "entry");

        self.builder.position_at_end(basic_block);

        let x = function.get_nth_param(0)?.into_int_value();
        let y = function.get_nth_param(1)?.into_int_value();
        let z = function.get_nth_param(2)?.into_int_value();

        let sum = self.builder.build_int_add(x, y, "sum");
        let sum = self.builder.build_int_add(sum, z, "sum");

        self.builder.build_return(Some(&sum));

        Some(())
    }

    pub fn write_to_string(&self) -> String {
        self.module.print_to_string().to_string()
    }

    pub fn write_output_file<P: AsRef<Path>>(
        &self,
        optimization: u8,
        target_triple: &str,
        path: P,
    ) -> anyhow::Result<()> {
        let opt_level = match optimization {
            0 => OptimizationLevel::None,       // -O0
            1 => OptimizationLevel::Less,       // -O1
            2 => OptimizationLevel::Default,    // -O2, -Os
            _ => OptimizationLevel::Aggressive, // -O3
        };
        let target_triple = TargetTriple::create(target_triple);
        let target = Target::from_triple(&target_triple).map_err(error::CodeGenError::from)?;
        let target_machine = target
            .create_target_machine(
                &target_triple,
                "generic",
                "",
                opt_level,
                RelocMode::Default,
                CodeModel::Default,
            )
            .ok_or(error::CodeGenError::CouldNotCreateTargetMachine)?;

        target_machine
            .write_to_file(&self.module, FileType::Object, path.as_ref())
            .map_err(error::CodeGenError::from)?;

        Ok(())
    }
}

pub fn run(optimization: u8, target_triple: &str) -> anyhow::Result<String> {
    Target::initialize_all(&Default::default());
    let context = Context::create();
    let mut codegen = CodeGen::new(&context);
    codegen.compile_example();

    codegen.write_output_file(optimization, target_triple, "module.o")?;

    Ok(codegen.write_to_string())
}
