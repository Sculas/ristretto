use crate::arguments::Arguments;
use crate::java_object::JavaObject;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Error::InternalError;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::{Reference, Value};
use std::sync::Arc;

/// Register all native methods for `sun.reflect.Reflection`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/reflect/Reflection";
    registry.register(
        class_name,
        "getCallerClass",
        "()Ljava/lang/Class;",
        get_caller_class_1,
    );
    registry.register(
        class_name,
        "getCallerClass",
        "(I)Ljava/lang/Class;",
        get_caller_class_2,
    );
    registry.register(
        class_name,
        "getClassAccessFlags",
        "(Ljava/lang/Class;)I",
        get_class_access_flags,
    );
}

#[async_recursion(?Send)]
async fn get_caller_class_1(thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    let frames = thread.frames().await?;
    let Some(frame) = frames.last() else {
        return Ok(Some(Value::Object(None)));
    };
    let class = frame.class();
    let class_name = class.name();
    let vm = thread.vm()?;
    let class = thread.class(class_name).await?;
    let class = class.to_object(&vm).await?;
    Ok(Some(class))
}

#[async_recursion(?Send)]
async fn get_caller_class_2(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.reflect.Reflection.getCallerClass(I)Ljava/lang/Class;")
}

#[async_recursion(?Send)]
async fn get_class_access_flags(
    thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let Some(Reference::Object(object)) = arguments.pop_reference()? else {
        return Err(InternalError(
            "getClassAccessFlags: no arguments".to_string(),
        ));
    };
    let class_name: String = object.value("name")?.try_into()?;
    let class = thread.class(&class_name).await?;
    let class_file = class.class_file();
    let access_flags = &class_file.access_flags;
    #[expect(clippy::cast_lossless)]
    let class_access_flags = access_flags.bits() as i32;
    Ok(Some(Value::Int(class_access_flags)))
}
