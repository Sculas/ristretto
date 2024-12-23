use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::Value;
use std::sync::Arc;

const JAVA_8: Version = Version::Java8 { minor: 0 };

/// Register all native methods for `java.lang.StrictMath`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/lang/StrictMath";
    let java_version = registry.java_version();

    if java_version <= &JAVA_8 {
        registry.register(class_name, "cbrt", "(D)D", cbrt);
        registry.register(class_name, "exp", "(D)D", exp);
        registry.register(class_name, "hypot", "(DD)D", hypot);
        registry.register(class_name, "pow", "(DD)D", pow);
    }

    registry.register(class_name, "IEEEremainder", "(DD)D", iee_eremainder);
    registry.register(class_name, "acos", "(D)D", acos);
    registry.register(class_name, "asin", "(D)D", asin);
    registry.register(class_name, "atan", "(D)D", atan);
    registry.register(class_name, "atan2", "(DD)D", atan_2);
    registry.register(class_name, "cos", "(D)D", cos);
    registry.register(class_name, "cosh", "(D)D", cosh);
    registry.register(class_name, "expm1", "(D)D", expm_1);
    registry.register(class_name, "log", "(D)D", log);
    registry.register(class_name, "log10", "(D)D", log_10);
    registry.register(class_name, "log1p", "(D)D", log_1_p);
    registry.register(class_name, "sin", "(D)D", sin);
    registry.register(class_name, "sinh", "(D)D", sinh);
    registry.register(class_name, "sqrt", "(D)D", sqrt);
    registry.register(class_name, "tan", "(D)D", tan);
    registry.register(class_name, "tanh", "(D)D", tanh);
}

#[async_recursion(?Send)]
async fn iee_eremainder(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.StrictMath.IEEERemainder(DD)D")
}

#[async_recursion(?Send)]
async fn acos(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.StrictMath.acos(D)D")
}

#[async_recursion(?Send)]
async fn asin(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.StrictMath.asin(D)D")
}

#[async_recursion(?Send)]
async fn atan(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.StrictMath.atan(D)D")
}

#[async_recursion(?Send)]
async fn atan_2(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.StrictMath.atan2(DD)D")
}

#[async_recursion(?Send)]
async fn cbrt(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.StrictMath.cbrt(D)D")
}

#[async_recursion(?Send)]
async fn cos(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.StrictMath.cos(D)D")
}

#[async_recursion(?Send)]
async fn cosh(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.StrictMath.cosh(D)D")
}

#[async_recursion(?Send)]
async fn exp(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.StrictMath.exp(D)D")
}

#[async_recursion(?Send)]
async fn expm_1(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.StrictMath.expm1(D)D")
}

#[async_recursion(?Send)]
async fn hypot(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.StrictMath.hypot(DD)D")
}

#[async_recursion(?Send)]
async fn log(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.StrictMath.log(D)D")
}

#[async_recursion(?Send)]
async fn log_10(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.StrictMath.log10(D)D")
}

#[async_recursion(?Send)]
async fn log_1_p(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.StrictMath.log1p(D)D")
}

#[async_recursion(?Send)]
async fn pow(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.StrictMath.pow(DD)D")
}

#[async_recursion(?Send)]
async fn sin(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.StrictMath.sin(D)D")
}

#[async_recursion(?Send)]
async fn sinh(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.StrictMath.sinh(D)D")
}

#[async_recursion(?Send)]
async fn sqrt(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.StrictMath.sqrt(D)D")
}

#[async_recursion(?Send)]
async fn tan(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.StrictMath.tan(D)D")
}

#[async_recursion(?Send)]
async fn tanh(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.StrictMath.tanh(D)D")
}
