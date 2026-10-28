use std::collections::HashMap;

use inkwell::{builder::Builder, values::BasicValue};
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::IntValue;
use inkwell::OptimizationLevel;

use crate::ast::ASTNodeType::{Assignment, IdenDeclarator};
use crate::{ast::{ASTNode, ASTNodeType}, runtime::{self, RuntimeValue}, symbol_table::{self, Symbol, SymbolTable}, token::Token};

pub struct Codegen<'ctx> {
    pub context: &'ctx Context,
    pub module: Module<'ctx>,
    pub builder: Builder<'ctx>
}

impl <'ctx> Codegen<'ctx> {
    pub fn compile_expr (&self, expr: &ASTNode) -> IntValue<'ctx> {
        match expr._type {
            ASTNodeType::Literal => {
                match &expr.value.value {
                    RuntimeValue::String(s) => {
                        self.context.i32_type().const_int(s.parse::<u64>().unwrap(), false)
                    },
                    RuntimeValue::Int(i) => {
                        self.context.i32_type().const_int(*i as u64, false)
                    }
                    _ => panic!("unsupported literal value"),
                }
            },
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
            _ => {
                    match &expr.value.value {
                    RuntimeValue::String(s) => {
                        self.context.i32_type().const_int(s.parse::<u64>().unwrap(), false)
                    }
                    _ => panic!("unsupported literal value"),
                }
            }
        }
    }
    
    pub fn compile_statement(&self, statement: &ASTNode, mut symb_table: symbol_table::SymbolTable) -> Option<IntValue<'ctx>>{
        match statement._type {
            ASTNodeType::BinaryExpression | ASTNodeType::Literal => {
                return Some(self.compile_expr(statement))
            }
            _ => {}
        }

        if matches!(statement._type, IdenDeclarator) {
            let mut symbol = symbol_table::Symbol::new();
            if let RuntimeValue::String(ref s) = statement.value.value {
                match s.clone().as_str() {
                    "def" => {
                        symbol.mutability = false;
                    },
                    _ => {

                    }
                }
                let assignment = statement.left_child.clone().expect("");
                if matches!(assignment._type, Assignment) {
                    let identifier = assignment.left_child.expect("").clone();
                    let expression = self.compile_expr(&assignment.right_child.expect("None"));  

                    if let RuntimeValue::String(ref s) = identifier.value.value {
                        symbol.name = s.clone();
                        //replace with type-checking
                        if let Some(val) = expression.get_sign_extended_constant() {
                            symbol.value = RuntimeValue::Int(val);
                            if let Some(curr) = symb_table.scope_stack.last_mut() {
                                curr.insert(symbol.name.clone(), symbol);
                                println!("{:?}", symb_table.scope_stack.last_mut().expect("").iter().last());
                                return None
                            }
                        }
                    }
                    //error handling 
                }
                //error handling
            }
            // error handling
        }
        // error handling
        None
    }
    
   pub fn compile_program(&self, ast: &ASTNode, symb_table: symbol_table::SymbolTable) {
        let i32_type = self.context.i32_type();
        
        // Define a function that takes no arguments and returns an i32: fn main() -> i32
        let fn_type = i32_type.fn_type(&[], false);
        let function = self.module.add_function("jit_main", fn_type, None);

        // Create the entry block and position the builder
        let basic_block = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(basic_block);

        let result_value = self.compile_statement(ast, symb_table);
        let return_arg = result_value.as_ref().map(|v| v as &dyn BasicValue);

        if !matches!(result_value, None){
            self.builder.build_return(return_arg).unwrap();
        } else {
            self.builder.build_return(Some(&i32_type.const_int(1, false))).unwrap();
        }
    }
    pub fn exec(expr: &ASTNode, symb_table: symbol_table::SymbolTable) -> Result<(), Box<dyn std::error::Error>>{
            
        let context = Context::create();
        let codegen = Codegen {
            context: &context,
            module: context.create_module("my_jit_module"),
            builder: context.create_builder(),
        };

        codegen.compile_program(expr, symb_table);

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


