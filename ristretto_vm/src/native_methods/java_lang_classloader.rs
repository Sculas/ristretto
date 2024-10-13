use crate::arguments::Arguments;
use crate::call_stack::CallStack;
use crate::native_methods::registry::MethodRegistry;
use crate::Result;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for java.lang.ClassLoader.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/lang/ClassLoader";
    registry.register(class_name, "registerNatives", "()V", register_natives);
}

#[expect(clippy::needless_pass_by_value)]
#[expect(clippy::unnecessary_wraps)]
fn register_natives(_call_stack: &Arc<CallStack>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}
