use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::Value;
use std::sync::Arc;

const JAVA_17: Version = Version::Java17 { minor: 0 };

/// Register all native methods for `sun.java2d.opengl.CGLGraphicsConfig`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/java2d/opengl/CGLGraphicsConfig";
    let java_version = registry.java_version();

    if java_version <= &JAVA_17 {
        registry.register(
            class_name,
            "getCGLConfigInfo",
            "(III)J",
            get_cgl_config_info,
        );
    } else {
        registry.register(class_name, "getCGLConfigInfo", "()J", get_cgl_config_info);
    }

    registry.register(
        class_name,
        "getOGLCapabilities",
        "(J)I",
        get_ogl_capabilities,
    );
    registry.register(class_name, "initCGL", "()Z", init_cgl);
    registry.register(
        class_name,
        "nativeGetMaxTextureSize",
        "()I",
        native_get_max_texture_size,
    );
}

#[async_recursion(?Send)]
async fn get_cgl_config_info(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.opengl.CGLGraphicsConfig.getCGLConfigInfo(III)J")
}

#[async_recursion(?Send)]
async fn get_ogl_capabilities(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.java2d.opengl.CGLGraphicsConfig.getOGLCapabilities(J)I")
}

#[async_recursion(?Send)]
async fn init_cgl(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.opengl.CGLGraphicsConfig.initCGL()Z")
}

#[async_recursion(?Send)]
async fn native_get_max_texture_size(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.java2d.opengl.CGLGraphicsConfig.nativeGetMaxTextureSize()I")
}
