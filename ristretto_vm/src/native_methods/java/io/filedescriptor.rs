use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::Value;
use std::sync::Arc;

const JAVA_11: Version = Version::Java11 { minor: 0 };
const JAVA_20: Version = Version::Java20 { minor: 0 };

/// Register all native methods for `java.io.FileDescriptor`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/io/FileDescriptor";
    let java_version = registry.java_version().clone();

    if java_version >= JAVA_11 {
        registry.register(class_name, "close0", "()V", close_0);
        registry.register(class_name, "getAppend", "(I)Z", get_append);
        registry.register(class_name, "getHandle", "(I)J", get_handle);
    }

    if java_version <= JAVA_20 {
        registry.register(class_name, "sync", "()V", sync);
    } else {
        registry.register(class_name, "sync0", "()V", sync_0);
    }

    registry.register(class_name, "initIDs", "()V", init_ids);
    registry.register(class_name, "sync", "()V", sync);
}

#[async_recursion(?Send)]
async fn close_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.io.FileDescriptor.close0()V")
}

#[expect(clippy::match_same_arms)]
#[async_recursion(?Send)]
async fn get_append(_thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let handle = arguments.pop_int()?;
    let append = match handle {
        0 => {
            // true if stdin is in append mode
            false
        }
        1 => {
            // true if stdout is in append mode
            false
        }
        2 => {
            // true if stderr is in append mode
            false
        }
        _ => false,
    };
    Ok(Some(Value::from(append)))
}

#[async_recursion(?Send)]
async fn get_handle(_thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let handle = arguments.pop_int()?;
    let handle = i64::from(handle);
    Ok(Some(Value::Long(handle)))
}

#[async_recursion(?Send)]
async fn init_ids(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn sync(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.io.FileDescriptor.sync()V")
}

#[async_recursion(?Send)]
async fn sync_0(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    sync(thread, arguments).await
}
