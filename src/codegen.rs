use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::IntValue;
use inkwell::OptimizationLevel;

use crate::{ast::{ASTNode, ASTNodeType}, token::Token};

pub struct Codegen<'ctx> {
    pub context: &'ctx Context,
    pub module: Module<'ctx>,
    pub builder: Builder<'ctx>
}

impl <'ctx> Codegen<'ctx> {
    pub fn compile_expr (&self, expr: &ASTNode) -> IntValue<'ctx> {
        match expr._type {
            ASTNodeType::Literal => self.context.i32_type().const_int(expr.value.value.parse::<u64>().unwrap(), false),
            ASTNodeType::BinaryExpression => {
                let left = self.compile_expr(expr.left_child.as_deref().unwrap());
                let right = self.compile_expr(expr.right_child.as_deref().unwrap());

                match expr.value._type {
                    Token::PLUS => self.builder.build_int_add(left, right, "addtmp").unwrap(),
                    Token::MINUS => self.builder.build_int_sub(left, right, "subtmp").unwrap(),
                    Token::DIV => self.builder.build_int_signed_div(left, right, "divtmp").unwrap(),
                    Token::MUL => self.builder.build_int_mul(left, right, "multmp").unwrap(),
                    _ => {self.builder.build_int_add(left, right, "addtmp").unwrap()}
                }
            },
            _ => self.context.i32_type().const_int(expr.value.value.parse::<u64>().unwrap(), false)
        }
    }
   pub fn compile_program(&self, ast: &ASTNode) {
        let i32_type = self.context.i32_type();
        
        // Define a function that takes no arguments and returns an i32: fn main() -> i32
        let fn_type = i32_type.fn_type(&[], false);
        let function = self.module.add_function("jit_main", fn_type, None);

        // Create the entry block and position the builder
        let basic_block = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(basic_block);

        let result_value = self.compile_expr(ast);

        self.builder.build_return(Some(&result_value)).unwrap();
    }
    pub fn exec(expr: &ASTNode) -> Result<(), Box<dyn std::error::Error>>{
            
        let context = Context::create();
        let codegen = Codegen {
            context: &context,
            module: context.create_module("my_jit_module"),
            builder: context.create_builder(),
        };

        codegen.compile_program(expr);

        /* 
        println!("--- Generated LLVM IR ---");
        codegen.module.print_to_stderr();
        println!("-------------------------\n");
        */

        let execution_engine = codegen
            .module
            .create_jit_execution_engine(OptimizationLevel::None)
            .map_err(|e| e.to_string())?;

        
        unsafe {
            let jit_main_fn = execution_engine
                .get_function::<unsafe extern "C" fn() -> i32>("jit_main")?;
            
            let runtime_result = jit_main_fn.call();
            println!("{}", runtime_result);
        Ok(())
        }
    }
}


