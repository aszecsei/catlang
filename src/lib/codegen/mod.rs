use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::OptimizationLevel;

pub struct CodeGen<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
}

impl<'ctx> CodeGen<'ctx> {
    pub fn new(context: &'ctx Context, optimization: u8) -> Self {
        let opt_level = match optimization {
            0 => OptimizationLevel::None,       // -O0
            1 => OptimizationLevel::Less,       // -O1
            2 => OptimizationLevel::Default,    // -O2, -Os
            _ => OptimizationLevel::Aggressive, // -O3
        };

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
}

pub fn run(optimization: u8) -> String {
    let context = Context::create();
    let mut codegen = CodeGen::new(&context, optimization);
    codegen.compile_example();
    codegen.write_to_string()
}
