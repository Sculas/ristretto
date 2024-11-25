use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.java2d.metal.MTLLayer`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/java2d/metal/MTLLayer";
    registry.register(class_name, "blitTexture", "(J)V", blit_texture);
    registry.register(class_name, "nativeCreateLayer", "()J", native_create_layer);
    registry.register(class_name, "nativeSetInsets", "(JII)V", native_set_insets);
    registry.register(class_name, "nativeSetScale", "(JD)V", native_set_scale);
    registry.register(
        class_name,
        "validate",
        "(JLsun/java2d/metal/MTLSurfaceData;)V",
        validate,
    );
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn blit_texture(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn native_create_layer(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn native_set_insets(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn native_set_scale(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn validate(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}
