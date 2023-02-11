

extern crate llvm_sys;

use llvm_sys::bit_writer::*;
use llvm_sys::core::*;
use std::ptr;
use std::process::Command;

macro_rules! c_str {
    ($s:expr) => (
        concat!($s, "\0").as_ptr() as *const i8
    );
}

fn main() {
    unsafe {
        // setup
        let context = LLVMContextCreate();
        let module = LLVMModuleCreateWithName(c_str!("main"));
        let builder = LLVMCreateBuilderInContext(context);

        // common types
        let void_type = LLVMVoidTypeInContext(context);
        let i8_type = LLVMIntTypeInContext(context, 8);
        let i8_pointer_type = LLVMPointerType(i8_type, 0);

        // declare that there's a `void log(i8*)` function in the environment
        // but don't provide a block (aka body) so that it in the wasm module
        // it'll be imported
        let log_func_type = LLVMFunctionType(void_type, [i8_pointer_type,].as_ptr() as *mut _, 1, 1);
        let log_func = LLVMAddFunction(module, c_str!("printf"), log_func_type);

        // our "main" function which will be the entry point when we run the executable
        let main_func_type = LLVMFunctionType(void_type, ptr::null_mut(), 0, 0);
        let main_func = LLVMAddFunction(module, c_str!("main"), main_func_type);
        let main_block = LLVMAppendBasicBlockInContext(context, main_func, c_str!("main"));
        LLVMPositionBuilderAtEnd(builder, main_block);

        // main's function body
        let world_str = LLVMBuildGlobalStringPtr(builder, c_str!("world!"), c_str!(""));
        let hello_world_str = LLVMBuildGlobalStringPtr(builder, c_str!("Hello %s\n"), c_str!(""));

        let log_args = [hello_world_str, world_str].as_ptr() as *mut _;
        // calling `printf("Hello %s!", "world")`
        LLVMBuildCall(builder, log_func, log_args, 2, c_str!(""));
        LLVMBuildRetVoid(builder);

        // write our bitcode file to arm64
        LLVMSetTarget(module, c_str!("arm64"));
        LLVMWriteBitcodeToFile(module, c_str!("main.bc"));

        // clean up
        LLVMDisposeBuilder(builder);
        LLVMDisposeModule(module);
        LLVMContextDispose(context);
    }

    // Run clang
    Command::new("clang")
    .arg("main.bc")
    .arg("-o")
    .arg("main")
    .output()
    .expect("Failed to execute clang with main.bc file");

    println!("main executable generated, run with ./main")
    
}