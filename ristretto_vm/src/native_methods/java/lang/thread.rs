use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::JavaError::NullPointerException;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::{Object, Reference, Value};
use std::sync::Arc;
use std::time::Duration;

const JAVA_11: Version = Version::Java11 { minor: 0 };
const JAVA_18: Version = Version::Java18 { minor: 0 };
const JAVA_19: Version = Version::Java19 { minor: 0 };
const JAVA_20: Version = Version::Java20 { minor: 0 };
const JAVA_21: Version = Version::Java21 { minor: 0 };
const JAVA_22: Version = Version::Java22 { minor: 0 };

/// Register all native methods for `java.lang.Thread`.
#[expect(clippy::too_many_lines)]
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/lang/Thread";
    let java_version = registry.java_version().clone();

    if java_version <= JAVA_11 || java_version == JAVA_18 {
        registry.register(class_name, "countStackFrames", "()I", count_stack_frames);
        registry.register(class_name, "isAlive", "()Z", is_alive);
        registry.register(class_name, "isInterrupted", "(Z)Z", is_interrupted);
    } else {
        registry.register(
            class_name,
            "clearInterruptEvent",
            "()V",
            clear_interrupt_event,
        );
    }

    if java_version <= JAVA_19 {
        registry.register(class_name, "resume0", "()V", resume_0);
    }
    if java_version == JAVA_19 {
        registry.register(
            class_name,
            "extentLocalCache",
            "()[Ljava/lang/Object;",
            extent_local_cache,
        );
    }
    if java_version >= JAVA_19 {
        registry.register(
            class_name,
            "currentCarrierThread",
            "()Ljava/lang/Thread;",
            current_carrier_thread,
        );
        registry.register(
            class_name,
            "getNextThreadIdOffset",
            "()J",
            get_next_thread_id_offset,
        );
        registry.register(
            class_name,
            "getStackTrace0",
            "()Ljava/lang/Object;",
            get_stack_trace_0,
        );

        if java_version <= JAVA_20 {
            registry.register(class_name, "isAlive0", "()Z", is_alive_0);
        }

        registry.register(
            class_name,
            "setCurrentThread",
            "(Ljava/lang/Thread;)V",
            set_current_thread,
        );
        registry.register(
            class_name,
            "setExtentLocalCache",
            "([Ljava/lang/Object;)V",
            set_extent_local_cache,
        );

        if java_version <= JAVA_21 {
            registry.register(class_name, "sleep0", "(J)V", sleep_0);
        }

        registry.register(class_name, "yield0", "()V", yield_0);
    }

    if java_version >= JAVA_20 {
        registry.register(
            class_name,
            "ensureMaterializedForStackWalk",
            "(Ljava/lang/Object;)V",
            ensure_materialized_for_stack_walk,
        );
        registry.register(
            class_name,
            "findScopedValueBindings",
            "()Ljava/lang/Object;",
            find_scoped_value_bindings,
        );
        registry.register(
            class_name,
            "scopedValueCache",
            "()[Ljava/lang/Object;",
            scoped_value_cache,
        );
        registry.register(
            class_name,
            "setScopedValueCache",
            "([Ljava/lang/Object;)V",
            set_scoped_value_cache,
        );
    }

    if java_version >= JAVA_22 {
        registry.register(class_name, "sleepNanos0", "(J)V", sleep_nanos_0);
    }

    registry.register(
        class_name,
        "currentThread",
        "()Ljava/lang/Thread;",
        current_thread,
    );
    registry.register(
        class_name,
        "dumpThreads",
        "([Ljava/lang/Thread;)[[Ljava/lang/StackTraceElement;",
        dump_threads,
    );
    registry.register(
        class_name,
        "getThreads",
        "()[Ljava/lang/Thread;",
        get_threads,
    );
    registry.register(class_name, "holdsLock", "(Ljava/lang/Object;)Z", holds_lock);
    registry.register(class_name, "interrupt0", "()V", interrupt_0);
    registry.register(class_name, "registerNatives", "()V", register_natives);
    registry.register(
        class_name,
        "setNativeName",
        "(Ljava/lang/String;)V",
        set_native_name,
    );
    registry.register(class_name, "setPriority0", "(I)V", set_priority_0);
    registry.register(class_name, "start0", "()V", start_0);
    registry.register(class_name, "stop0", "(Ljava/lang/Object;)V", stop_0);
    registry.register(class_name, "suspend0", "()V", suspend_0);
}

#[async_recursion(?Send)]
async fn clear_interrupt_event(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn count_stack_frames(thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    let frames = thread.frames().await?;
    let frames = i32::try_from(frames.len())?;
    Ok(Some(Value::Int(frames)))
}

#[async_recursion(?Send)]
async fn current_carrier_thread(
    thread: Arc<Thread>,
    arguments: Arguments,
) -> Result<Option<Value>> {
    // TODO: correct this once threading is implemented
    current_thread(thread, arguments).await
}

#[async_recursion(?Send)]
async fn current_thread(thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    let thread = thread.java_object().await;
    Ok(Some(thread))
}

#[async_recursion(?Send)]
async fn dump_threads(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.Thread.dumpThreads([Ljava/lang/Thread;)[[Ljava/lang/StackTraceElement;")
}

#[async_recursion(?Send)]
async fn ensure_materialized_for_stack_walk(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("java.lang.Thread.ensureMaterializedForStackWalk(Ljava/lang/Object;)V")
}

#[async_recursion(?Send)]
async fn extent_local_cache(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.Thread.extentLocalCache()[Ljava/lang/Object;")
}

#[async_recursion(?Send)]
async fn find_scoped_value_bindings(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("java.lang.Thread.findScopedValueBindings()Ljava/lang/Object;")
}

#[async_recursion(?Send)]
async fn get_next_thread_id_offset(
    thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    let vm = thread.vm()?;
    let thread_id = vm.next_thread_id()?;
    let thread_id = i64::try_from(thread_id)?;
    Ok(Some(Value::from(thread_id)))
}

#[async_recursion(?Send)]
async fn get_stack_trace_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.Thread.getStackTrace0()Ljava/lang/Object;")
}

#[async_recursion(?Send)]
async fn get_threads(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.Thread.getThreads()[Ljava/lang/Thread;")
}

#[async_recursion(?Send)]
async fn holds_lock(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.Thread.holdsLock(Ljava/lang/Object;)Z")
}

#[async_recursion(?Send)]
async fn interrupt_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.Thread.interrupt0()V")
}

#[async_recursion(?Send)]
async fn is_alive(thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    let object: Object = thread.java_object().await.try_into()?;
    let eetop = object.value("eetop")?.to_long()?;
    let is_alive = eetop != 0;
    Ok(Some(Value::from(is_alive)))
}

#[async_recursion(?Send)]
async fn is_alive_0(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    is_alive(thread, arguments).await
}

#[async_recursion(?Send)]
async fn is_interrupted(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.Thread.isInterrupted(Z)Z")
}

#[async_recursion(?Send)]
async fn register_natives(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn resume_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.Thread.resume0()V")
}

#[async_recursion(?Send)]
async fn scoped_value_cache(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.Thread.scopedValueCache()[Ljava/lang/Object;")
}

#[async_recursion(?Send)]
async fn set_current_thread(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.Thread.setCurrentThread(Ljava/lang/Thread;)V")
}

#[async_recursion(?Send)]
async fn set_extent_local_cache(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn set_native_name(thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let Some(Reference::Object(name)) = arguments.pop_reference()? else {
        return Err(NullPointerException("name cannot be null".to_string()).into());
    };
    let name: String = name.try_into()?;
    thread.set_name(name).await;
    Ok(None)
}

#[async_recursion(?Send)]
async fn set_priority_0(_thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let _new_priority = arguments.pop_int()?;
    // TODO: implement priority if/when tokio supports it
    Ok(None)
}

#[async_recursion(?Send)]
async fn set_scoped_value_cache(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("java.lang.Thread.setScopedValueCache([Ljava/lang/Object;)V")
}

#[async_recursion(?Send)]
async fn sleep(_thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let millis = arguments.pop_long()?;
    let millis = u64::try_from(millis)?;
    let duration = Duration::from_millis(millis);
    #[cfg(not(target_arch = "wasm32"))]
    tokio::time::sleep(duration).await;
    #[cfg(target_arch = "wasm32")]
    std::thread::sleep(duration);
    Ok(None)
}

#[async_recursion(?Send)]
async fn sleep_0(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    sleep(thread, arguments).await
}

#[async_recursion(?Send)]
async fn sleep_nanos_0(_thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let nanos = arguments.pop_long()?;
    let nanos = u64::try_from(nanos)?;
    let duration = Duration::from_nanos(nanos);
    #[cfg(not(target_arch = "wasm32"))]
    tokio::time::sleep(duration).await;
    #[cfg(target_arch = "wasm32")]
    std::thread::sleep(duration);
    Ok(None)
}

#[async_recursion(?Send)]
async fn start_0(thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    let thread_id = i64::try_from(thread.id())?;
    let object: Object = thread.java_object().await.try_into()?;
    object.set_value("eetop", Value::from(thread_id))?;
    Ok(None)
}

#[async_recursion(?Send)]
async fn stop_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.Thread.stop0(Ljava/lang/Object;)V")
}

#[async_recursion(?Send)]
async fn suspend_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.Thread.suspend0()V")
}

#[async_recursion(?Send)]
async fn r#yield(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    #[cfg(not(target_arch = "wasm32"))]
    tokio::task::yield_now().await;
    #[cfg(target_arch = "wasm32")]
    std::thread::yield_now();
    Ok(None)
}

#[async_recursion(?Send)]
async fn yield_0(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    r#yield(thread, arguments).await
}
