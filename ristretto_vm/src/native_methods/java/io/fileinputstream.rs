use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::Value;
use std::sync::Arc;

const JAVA_8: Version = Version::Java8 { minor: 0 };
const JAVA_17: Version = Version::Java17 { minor: 0 };

/// Register all native methods for `java.io.FileInputStream`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/io/FileInputStream";
    let java_version = registry.java_version();

    if java_version <= &JAVA_8 {
        registry.register(class_name, "close0", "()V", close_0);
    } else if java_version >= &JAVA_17 {
        registry.register(class_name, "length0", "()J", length_0);
        registry.register(class_name, "position0", "()J", position_0);
    }

    registry.register(class_name, "available0", "()I", available_0);
    registry.register(class_name, "initIDs", "()V", init_ids);
    registry.register(class_name, "open0", "(Ljava/lang/String;)V", open_0);
    registry.register(class_name, "read0", "()I", read_0);
    registry.register(class_name, "readBytes", "([BII)I", read_bytes);
    registry.register(class_name, "skip0", "(J)J", skip_0);
}

#[async_recursion(?Send)]
async fn available_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.io.FileInputStream.available0()I")
}

#[async_recursion(?Send)]
async fn close_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.io.FileInputStream.close0()V")
}

#[async_recursion(?Send)]
async fn init_ids(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn length_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.io.FileInputStream.length0()J")
}

#[async_recursion(?Send)]
async fn open_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.io.FileInputStream.open0(Ljava/lang/String;)V")
}

#[async_recursion(?Send)]
async fn position_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.io.FileInputStream.position0()J")
}

#[async_recursion(?Send)]
async fn read_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.io.FileInputStream.read0()I")
}

#[async_recursion(?Send)]
async fn read_bytes(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.io.FileInputStream.readBytes([BII)I")
}

#[async_recursion(?Send)]
async fn skip_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.io.FileInputStream.skip0(J)J")
}
