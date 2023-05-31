use std::path::Path;

use inkwell::{
    builder::Builder,
    context::Context,
    module::Module,
    targets::{CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine},
    OptimizationLevel,
};

pub fn compile(typed_ast: (), path: &Path) {
    Codegen::compile(typed_ast, path)
}

struct Codegen<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
}

impl<'ctx> Codegen<'ctx> {
    pub fn compile(ast: (), path: &Path) {
        let context = Context::create();
        let module = context.create_module("exofork");
        let builder = context.create_builder();
        let mut codegen = Codegen {
            context: &context,
            module,
            builder,
        };
        codegen.compile_ast(ast);
        codegen.write(path);
    }

    fn write(&mut self, path: &Path) {
        Target::initialize_aarch64(&InitializationConfig::default());
        Target::initialize_x86(&InitializationConfig::default());
        let triple = TargetMachine::get_default_triple();
        let target = Target::from_triple(&triple).unwrap();
        let cpu = TargetMachine::get_host_cpu_name();
        let features = TargetMachine::get_host_cpu_features();
        let reloc = RelocMode::Default;
        let model = CodeModel::Default;
        let opt = OptimizationLevel::Aggressive;
        let target_machine = target
            .create_target_machine(
                &triple,
                cpu.to_str().unwrap(),
                features.to_str().unwrap(),
                opt,
                reloc,
                model,
            )
            .unwrap();

        target_machine
            .write_to_file(&self.module, FileType::Object, path)
            .unwrap();
    }

    fn compile_ast(&mut self, ast: ()) {}
}
