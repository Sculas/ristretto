use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `java.lang.Compiler`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/lang/Compiler";
    registry.register(
        class_name,
        "command",
        "(Ljava/lang/Object;)Ljava/lang/Object;",
        command,
    );
    registry.register(
        class_name,
        "compileClass",
        "(Ljava/lang/Class;)Z",
        compile_class,
    );
    registry.register(
        class_name,
        "compileClasses",
        "(Ljava/lang/String;)Z",
        compile_classes,
    );
    registry.register(class_name, "disable", "()V", disable);
    registry.register(class_name, "enable", "()V", enable);
    registry.register(class_name, "initialize", "()V", initialize);
    registry.register(class_name, "registerNatives", "()V", register_natives);
}

#[async_recursion(?Send)]
async fn command(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.Compiler.command(Ljava/lang/Object;)Ljava/lang/Object;")
}

#[async_recursion(?Send)]
async fn compile_class(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.Compiler.compileClass(Ljava/lang/Class;)Z")
}

#[async_recursion(?Send)]
async fn compile_classes(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.Compiler.compileClasses(Ljava/lang/String;)Z")
}

#[async_recursion(?Send)]
async fn disable(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.Compiler.disable()V")
}

#[async_recursion(?Send)]
async fn enable(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.Compiler.enable()V")
}

#[async_recursion(?Send)]
async fn initialize(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn register_natives(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}
