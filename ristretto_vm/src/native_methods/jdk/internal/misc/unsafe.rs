use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Error::{InternalError, InvalidOperand};
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::{BaseType, Version};
use ristretto_classloader::{Reference, Value};
use std::sync::Arc;

const JAVA_11: Version = Version::Java11 { minor: 0 };
const JAVA_17: Version = Version::Java17 { minor: 0 };

/// Register all native methods for `jdk.internal.misc.Unsafe`.
#[expect(clippy::too_many_lines)]
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "jdk/internal/misc/Unsafe";
    let java_version = registry.java_version().clone();

    if java_version <= JAVA_11 {
        registry.register(class_name, "addressSize0", "()I", address_size_0);
        registry.register(
            class_name,
            "compareAndExchangeObject",
            "(Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            compare_and_exchange_object,
        );
        registry.register(
            class_name,
            "compareAndSetObject",
            "(Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)Z",
            compare_and_set_object,
        );
        registry.register(
            class_name,
            "defineAnonymousClass0",
            "(Ljava/lang/Class;[B[Ljava/lang/Object;)Ljava/lang/Class;",
            define_anonymous_class_0,
        );
        registry.register(
            class_name,
            "getObject",
            "(Ljava/lang/Object;J)Ljava/lang/Object;",
            get_object,
        );
        registry.register(
            class_name,
            "getObjectVolatile",
            "(Ljava/lang/Object;J)Ljava/lang/Object;",
            get_object_volatile,
        );
        registry.register(class_name, "isBigEndian0", "()Z", is_big_endian_0);
        registry.register(class_name, "pageSize", "()I", page_size);
        registry.register(
            class_name,
            "putObject",
            "(Ljava/lang/Object;JLjava/lang/Object;)V",
            put_object,
        );
        registry.register(
            class_name,
            "putObjectVolatile",
            "(Ljava/lang/Object;JLjava/lang/Object;)V",
            put_object_volatile,
        );
        registry.register(class_name, "unalignedAccess0", "()Z", unaligned_access_0);
    } else {
        registry.register(
            class_name,
            "compareAndExchangeReference",
            "(Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            compare_and_exchange_reference,
        );
        registry.register(
            class_name,
            "compareAndSetReference",
            "(Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)Z",
            compare_and_set_reference,
        );
        registry.register(
            class_name,
            "getReference",
            "(Ljava/lang/Object;J)Ljava/lang/Object;",
            get_reference,
        );
        registry.register(
            class_name,
            "getReferenceVolatile",
            "(Ljava/lang/Object;J)Ljava/lang/Object;",
            get_reference_volatile,
        );
        registry.register(
            class_name,
            "getReference",
            "(Ljava/lang/Object;J)Ljava/lang/Object;",
            get_reference,
        );
        registry.register(
            class_name,
            "getReferenceVolatile",
            "(Ljava/lang/Object;J)Ljava/lang/Object;",
            get_reference_volatile,
        );
        registry.register(class_name, "writeback0", "(J)V", writeback_0);
        registry.register(
            class_name,
            "writebackPostSync0",
            "()V",
            writeback_post_sync_0,
        );
        registry.register(class_name, "writebackPreSync0", "()V", writeback_pre_sync_0);
    }

    if java_version <= JAVA_17 {
        registry.register(class_name, "loadFence", "()V", load_fence);
        registry.register(class_name, "storeFence", "()V", store_fence);
    }

    registry.register(
        class_name,
        "allocateInstance",
        "(Ljava/lang/Class;)Ljava/lang/Object;",
        allocate_instance,
    );
    registry.register(class_name, "allocateMemory0", "(J)J", allocate_memory_0);
    registry.register(
        class_name,
        "arrayBaseOffset0",
        "(Ljava/lang/Class;)I",
        array_base_offset_0,
    );
    registry.register(
        class_name,
        "arrayIndexScale0",
        "(Ljava/lang/Class;)I",
        array_index_scale_0,
    );
    registry.register(
        class_name,
        "compareAndExchangeInt",
        "(Ljava/lang/Object;JII)I",
        compare_and_exchange_int,
    );
    registry.register(
        class_name,
        "compareAndExchangeLong",
        "(Ljava/lang/Object;JJJ)J",
        compare_and_exchange_long,
    );
    registry.register(
        class_name,
        "compareAndExchangeReference",
        "(Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
        compare_and_exchange_reference,
    );
    registry.register(
        class_name,
        "compareAndSetInt",
        "(Ljava/lang/Object;JII)Z",
        compare_and_set_int,
    );
    registry.register(
        class_name,
        "compareAndSetLong",
        "(Ljava/lang/Object;JJJ)Z",
        compare_and_set_long,
    );
    registry.register(
        class_name,
        "compareAndSetReference",
        "(Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)Z",
        compare_and_set_reference,
    );
    registry.register(
        class_name,
        "copyMemory0",
        "(Ljava/lang/Object;JLjava/lang/Object;JJ)V",
        copy_memory_0,
    );
    registry.register(
        class_name,
        "copySwapMemory0",
        "(Ljava/lang/Object;JLjava/lang/Object;JJJ)V",
        copy_swap_memory_0,
    );
    registry.register(class_name, "defineClass0", "(Ljava/lang/String;[BIILjava/lang/ClassLoader;Ljava/security/ProtectionDomain;)Ljava/lang/Class;", define_class_0);
    registry.register(
        class_name,
        "ensureClassInitialized0",
        "(Ljava/lang/Class;)V",
        ensure_class_initialized_0,
    );
    registry.register(class_name, "freeMemory0", "(J)V", free_memory_0);
    registry.register(class_name, "fullFence", "()V", full_fence);
    registry.register(
        class_name,
        "getBoolean",
        "(Ljava/lang/Object;J)Z",
        get_boolean,
    );
    registry.register(
        class_name,
        "getBooleanVolatile",
        "(Ljava/lang/Object;J)Z",
        get_boolean_volatile,
    );
    registry.register(class_name, "getByte", "(Ljava/lang/Object;J)B", get_byte);
    registry.register(
        class_name,
        "getByteVolatile",
        "(Ljava/lang/Object;J)B",
        get_byte_volatile,
    );
    registry.register(class_name, "getChar", "(Ljava/lang/Object;J)C", get_char);
    registry.register(
        class_name,
        "getCharVolatile",
        "(Ljava/lang/Object;J)C",
        get_char_volatile,
    );
    registry.register(
        class_name,
        "getDouble",
        "(Ljava/lang/Object;J)D",
        get_double,
    );
    registry.register(
        class_name,
        "getDoubleVolatile",
        "(Ljava/lang/Object;J)D",
        get_double_volatile,
    );
    registry.register(class_name, "getFloat", "(Ljava/lang/Object;J)F", get_float);
    registry.register(
        class_name,
        "getFloatVolatile",
        "(Ljava/lang/Object;J)F",
        get_float_volatile,
    );
    registry.register(class_name, "getInt", "(Ljava/lang/Object;J)I", get_int);
    registry.register(
        class_name,
        "getIntVolatile",
        "(Ljava/lang/Object;J)I",
        get_int_volatile,
    );
    registry.register(class_name, "getLoadAverage0", "([DI)I", get_load_average_0);
    registry.register(class_name, "getLong", "(Ljava/lang/Object;J)J", get_long);
    registry.register(
        class_name,
        "getLongVolatile",
        "(Ljava/lang/Object;J)J",
        get_long_volatile,
    );
    registry.register(
        class_name,
        "getReference",
        "(Ljava/lang/Object;J)Ljava/lang/Object;",
        get_reference,
    );
    registry.register(
        class_name,
        "getReferenceVolatile",
        "(Ljava/lang/Object;J)Ljava/lang/Object;",
        get_reference_volatile,
    );
    registry.register(class_name, "getShort", "(Ljava/lang/Object;J)S", get_short);
    registry.register(
        class_name,
        "getShortVolatile",
        "(Ljava/lang/Object;J)S",
        get_short_volatile,
    );
    registry.register(
        class_name,
        "getUncompressedObject",
        "(J)Ljava/lang/Object;",
        get_uncompressed_object,
    );
    registry.register(
        class_name,
        "objectFieldOffset0",
        "(Ljava/lang/reflect/Field;)J",
        object_field_offset_0,
    );
    registry.register(
        class_name,
        "objectFieldOffset1",
        "(Ljava/lang/Class;Ljava/lang/String;)J",
        object_field_offset_1,
    );
    registry.register(class_name, "park", "(ZJ)V", park);
    registry.register(
        class_name,
        "putBoolean",
        "(Ljava/lang/Object;JZ)V",
        put_boolean,
    );
    registry.register(
        class_name,
        "putBooleanVolatile",
        "(Ljava/lang/Object;JZ)V",
        put_boolean_volatile,
    );
    registry.register(class_name, "putByte", "(Ljava/lang/Object;JB)V", put_byte);
    registry.register(
        class_name,
        "putByteVolatile",
        "(Ljava/lang/Object;JB)V",
        put_byte_volatile,
    );
    registry.register(class_name, "putChar", "(Ljava/lang/Object;JC)V", put_char);
    registry.register(
        class_name,
        "putCharVolatile",
        "(Ljava/lang/Object;JC)V",
        put_char_volatile,
    );
    registry.register(
        class_name,
        "putDouble",
        "(Ljava/lang/Object;JD)V",
        put_double,
    );
    registry.register(
        class_name,
        "putDoubleVolatile",
        "(Ljava/lang/Object;JD)V",
        put_double_volatile,
    );
    registry.register(class_name, "putFloat", "(Ljava/lang/Object;JF)V", put_float);
    registry.register(
        class_name,
        "putFloatVolatile",
        "(Ljava/lang/Object;JF)V",
        put_float_volatile,
    );
    registry.register(class_name, "putInt", "(Ljava/lang/Object;JI)V", put_int);
    registry.register(
        class_name,
        "putIntVolatile",
        "(Ljava/lang/Object;JI)V",
        put_int_volatile,
    );
    registry.register(class_name, "putLong", "(Ljava/lang/Object;JJ)V", put_long);
    registry.register(
        class_name,
        "putLongVolatile",
        "(Ljava/lang/Object;JJ)V",
        put_long_volatile,
    );
    registry.register(
        class_name,
        "putReference",
        "(Ljava/lang/Object;JLjava/lang/Object;)V",
        put_reference,
    );
    registry.register(
        class_name,
        "putReferenceVolatile",
        "(Ljava/lang/Object;JLjava/lang/Object;)V",
        put_reference_volatile,
    );
    registry.register(class_name, "putShort", "(Ljava/lang/Object;JS)V", put_short);
    registry.register(
        class_name,
        "putShortVolatile",
        "(Ljava/lang/Object;JS)V",
        put_short_volatile,
    );
    registry.register(
        class_name,
        "reallocateMemory0",
        "(JJ)J",
        reallocate_memory_0,
    );
    registry.register(class_name, "registerNatives", "()V", register_natives);
    registry.register(
        class_name,
        "setMemory0",
        "(Ljava/lang/Object;JJB)V",
        set_memory_0,
    );
    registry.register(
        class_name,
        "shouldBeInitialized0",
        "(Ljava/lang/Class;)Z",
        should_be_initialized_0,
    );
    registry.register(
        class_name,
        "staticFieldBase0",
        "(Ljava/lang/reflect/Field;)Ljava/lang/Object;",
        static_field_base_0,
    );
    registry.register(
        class_name,
        "staticFieldOffset0",
        "(Ljava/lang/reflect/Field;)J",
        static_field_offset_0,
    );
    registry.register(
        class_name,
        "throwException",
        "(Ljava/lang/Throwable;)V",
        throw_exception,
    );
    registry.register(class_name, "unpark", "(Ljava/lang/Object;)V", unpark);
    registry.register(class_name, "writeback0", "(J)V", writeback_0);
    registry.register(
        class_name,
        "writebackPostSync0",
        "()V",
        writeback_post_sync_0,
    );
    registry.register(class_name, "writebackPreSync0", "()V", writeback_pre_sync_0);
}

#[async_recursion(?Send)]
pub(crate) async fn address_size_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    Ok(Some(Value::Int(8))) // 64-bit pointers
}

#[async_recursion(?Send)]
pub(crate) async fn allocate_instance(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.Unsafe.allocateInstance(Ljava/lang/Class;)Ljava/lang/Object;")
}

#[async_recursion(?Send)]
pub(crate) async fn allocate_memory_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.Unsafe.allocateMemory0(J)J")
}

#[async_recursion(?Send)]
pub(crate) async fn array_base_offset_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    Ok(Some(Value::Int(0)))
}

#[async_recursion(?Send)]
pub(crate) async fn array_index_scale_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    Ok(Some(Value::Int(1)))
}

#[async_recursion(?Send)]
pub(crate) async fn compare_and_exchange_int(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.Unsafe.compareAndExchangeInt(Ljava/lang/Object;JII)I")
}

#[async_recursion(?Send)]
pub(crate) async fn compare_and_exchange_long(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.Unsafe.compareAndExchangeLong(Ljava/lang/Object;JJJ)J")
}

#[async_recursion(?Send)]
pub(crate) async fn compare_and_exchange_object(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.Unsafe.compareAndExchangeObject(Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;")
}

#[async_recursion(?Send)]
pub(crate) async fn compare_and_exchange_reference(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.Unsafe.compareAndExchangeReference(Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;")
}

#[async_recursion(?Send)]
pub(crate) async fn compare_and_set_int(
    _thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let x = arguments.pop_int()?;
    let expected = arguments.pop_int()?;
    let mut offset = arguments.pop()?;
    let Value::Long(ref mut offset) = offset else {
        return Err(InvalidOperand {
            expected: "long".to_string(),
            actual: offset.to_string(),
        });
    };

    // TODO: the compare and set operation should be atomic
    let result = if let Some(Reference::Object(object)) = arguments.pop_reference()? {
        let class = object.class();
        let offset = usize::try_from(*offset)?;
        let field_name = class.field_name(offset)?;
        let field = object.field(&field_name)?;
        let value = field.value()?.to_int()?;
        if value == expected {
            field.set_value(Value::Int(x))?;
            1
        } else {
            0
        }
    } else if i32::try_from(*offset)? == expected {
        *offset = i64::from(x);
        1
    } else {
        0
    };
    Ok(Some(Value::Int(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn compare_and_set_long(
    _thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let x = arguments.pop_long()?;
    let expected = arguments.pop_long()?;
    let mut offset = arguments.pop()?;
    let Value::Long(ref mut offset) = offset else {
        return Err(InvalidOperand {
            expected: "long".to_string(),
            actual: offset.to_string(),
        });
    };

    // TODO: the compare and set operation should be atomic
    let result = if let Some(Reference::Object(object)) = arguments.pop_reference()? {
        let class = object.class();
        let offset = usize::try_from(*offset)?;
        let field_name = class.field_name(offset)?;
        let field = object.field(&field_name)?;
        let value = field.value()?.to_long()?;
        if value == expected {
            field.set_value(Value::Long(x))?;
            1
        } else {
            0
        }
    } else if *offset == expected {
        *offset = x;
        1
    } else {
        0
    };
    Ok(Some(Value::Int(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn compare_and_set_object(
    thread: Arc<Thread>,
    arguments: Arguments,
) -> Result<Option<Value>> {
    compare_and_set_reference(thread, arguments).await
}

#[async_recursion(?Send)]
pub(crate) async fn compare_and_set_reference(
    _thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let x = arguments.pop()?;
    let expected = arguments.pop()?;
    let offset = arguments.pop_long()?;
    let offset = usize::try_from(offset)?;
    let Some(object) = arguments.pop_reference()? else {
        return Err(InternalError(
            "compareAndSetReference: Invalid reference".to_string(),
        ));
    };

    // TODO: the compare and set operation should be atomic
    let result = match object {
        Reference::Array(_class, array) => {
            let Some(reference) = array.get(offset)? else {
                return Err(InternalError(
                    "compareAndSetReference: Invalid reference index".to_string(),
                ));
            };
            let Value::Object(expected_reference) = expected else {
                return Err(InvalidOperand {
                    expected: "object".to_string(),
                    actual: expected.to_string(),
                });
            };

            if reference == expected_reference {
                let Value::Object(x_reference) = x else {
                    return Err(InvalidOperand {
                        expected: "object".to_string(),
                        actual: x.to_string(),
                    });
                };
                array.set(offset, x_reference)?;
                1
            } else {
                0
            }
        }
        Reference::Object(object) => {
            let field_name = object.class().field_name(offset)?;
            let field = object.field(&field_name)?;
            let value = field.value()?;
            if value == expected {
                field.set_value(x)?;
                1
            } else {
                0
            }
        }
        _ => {
            return Err(InternalError(
                "compareAndSetReference: Invalid reference".to_string(),
            ));
        }
    };
    Ok(Some(Value::Int(result)))
}

#[async_recursion(?Send)]
pub(crate) async fn copy_memory_0(
    _thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let _bytes = usize::try_from(arguments.pop_long()?)?;
    let _destination_offset = usize::try_from(arguments.pop_long()?)?;
    let Value::Object(ref mut destination) = arguments.pop()? else {
        return Err(InternalError(
            "copyMemory0: Invalid destination".to_string(),
        ));
    };
    let _source_offset = usize::try_from(arguments.pop_long()?)?;
    let Value::Object(ref mut source) = arguments.pop()? else {
        return Err(InternalError("copyMemory0: Invalid source".to_string()));
    };
    destination.clone_from(source);
    Ok(None)
}

#[async_recursion(?Send)]
pub(crate) async fn copy_swap_memory_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.Unsafe.copySwapMemory0(Ljava/lang/Object;JLjava/lang/Object;JJJ)V")
}

#[async_recursion(?Send)]
pub(crate) async fn define_anonymous_class_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.Unsafe.defineAnonymousClass0(Ljava/lang/Class;[B[Ljava/lang/Object;)Ljava/lang/Class;")
}

#[async_recursion(?Send)]
pub(crate) async fn define_class_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.Unsafe.defineClass0(Ljava/lang/String;[BIILjava/lang/ClassLoader;Ljava/security/ProtectionDomain;)Ljava/lang/Class;")
}

#[async_recursion(?Send)]
pub(crate) async fn ensure_class_initialized_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
pub(crate) async fn free_memory_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
pub(crate) async fn full_fence(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    Ok(None)
}

fn get_reference_type(
    _thread: Arc<Thread>,
    mut arguments: Arguments,
    base_type: Option<BaseType>,
) -> Result<Option<Value>> {
    let offset = arguments.pop_long()?;
    let Some(reference) = arguments.pop_reference()? else {
        let Some(base_type) = base_type else {
            return Err(InternalError(
                "getReferenceType: Invalid reference".to_string(),
            ));
        };
        let value = match base_type {
            BaseType::Boolean
            | BaseType::Byte
            | BaseType::Char
            | BaseType::Int
            | BaseType::Short => Value::Int(i32::try_from(offset)?),
            BaseType::Long => Value::Long(offset),
            BaseType::Double | BaseType::Float => {
                return Err(InternalError(
                    "getReferenceType: Invalid reference".to_string(),
                ));
            }
        };
        return Ok(Some(value));
    };

    let offset = usize::try_from(offset)?;
    match reference {
        Reference::Array(_class, array) => {
            let Some(reference) = array.get(offset)? else {
                return Err(InternalError(
                    "getReferenceType: Invalid reference index".to_string(),
                ));
            };
            Ok(Some(Value::Object(reference)))
        }
        Reference::Object(object) => {
            let field_name = object.class().field_name(offset)?;
            let value = object.value(&field_name)?;
            Ok(Some(value))
        }
        _ => Err(InternalError(
            "getReferenceType: Invalid reference".to_string(),
        )),
    }
}

#[async_recursion(?Send)]
pub(crate) async fn get_boolean(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.Unsafe.getBoolean(Ljava/lang/Object;J)Z")
}

#[async_recursion(?Send)]
pub(crate) async fn get_boolean_volatile(
    thread: Arc<Thread>,
    arguments: Arguments,
) -> Result<Option<Value>> {
    get_reference_type(thread, arguments, Some(BaseType::Boolean))
}

#[async_recursion(?Send)]
pub(crate) async fn get_byte(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.Unsafe.getByte(Ljava/lang/Object;J)B")
}

#[async_recursion(?Send)]
pub(crate) async fn get_byte_volatile(
    thread: Arc<Thread>,
    arguments: Arguments,
) -> Result<Option<Value>> {
    get_reference_type(thread, arguments, Some(BaseType::Byte))
}

#[async_recursion(?Send)]
pub(crate) async fn get_char(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.Unsafe.getChar(Ljava/lang/Object;J)C")
}

#[async_recursion(?Send)]
pub(crate) async fn get_char_volatile(
    thread: Arc<Thread>,
    arguments: Arguments,
) -> Result<Option<Value>> {
    get_reference_type(thread, arguments, Some(BaseType::Char))
}

#[async_recursion(?Send)]
pub(crate) async fn get_double(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.Unsafe.getDouble(Ljava/lang/Object;J)D")
}

#[async_recursion(?Send)]
pub(crate) async fn get_double_volatile(
    thread: Arc<Thread>,
    arguments: Arguments,
) -> Result<Option<Value>> {
    get_reference_type(thread, arguments, Some(BaseType::Double))
}

#[async_recursion(?Send)]
pub(crate) async fn get_float(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.Unsafe.getFloat(Ljava/lang/Object;J)F")
}

#[async_recursion(?Send)]
pub(crate) async fn get_float_volatile(
    thread: Arc<Thread>,
    arguments: Arguments,
) -> Result<Option<Value>> {
    get_reference_type(thread, arguments, Some(BaseType::Float))
}

#[async_recursion(?Send)]
pub(crate) async fn get_int(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.Unsafe.getInt(Ljava/lang/Object;J)I")
}

#[async_recursion(?Send)]
pub(crate) async fn get_int_volatile(
    thread: Arc<Thread>,
    arguments: Arguments,
) -> Result<Option<Value>> {
    get_reference_type(thread, arguments, Some(BaseType::Int))
}

#[async_recursion(?Send)]
pub(crate) async fn get_load_average_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.Unsafe.getLoadAverage0([DI)I")
}

#[async_recursion(?Send)]
pub(crate) async fn get_long(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.Unsafe.getLong(Ljava/lang/Object;J)J")
}

#[async_recursion(?Send)]
pub(crate) async fn get_long_volatile(
    thread: Arc<Thread>,
    arguments: Arguments,
) -> Result<Option<Value>> {
    get_reference_type(thread, arguments, Some(BaseType::Long))
}

#[async_recursion(?Send)]
pub(crate) async fn get_object(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.Unsafe.getObject(Ljava/lang/Object;J)Ljava/lang/Object;")
}

#[async_recursion(?Send)]
pub(crate) async fn get_object_volatile(
    thread: Arc<Thread>,
    arguments: Arguments,
) -> Result<Option<Value>> {
    get_reference_type(thread, arguments, None)
}

#[async_recursion(?Send)]
pub(crate) async fn get_reference(
    thread: Arc<Thread>,
    arguments: Arguments,
) -> Result<Option<Value>> {
    get_reference_type(thread, arguments, None)
}

#[async_recursion(?Send)]
pub(crate) async fn get_reference_volatile(
    thread: Arc<Thread>,
    arguments: Arguments,
) -> Result<Option<Value>> {
    get_reference_type(thread, arguments, None)
}

#[async_recursion(?Send)]
pub(crate) async fn get_short(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.Unsafe.getShort(Ljava/lang/Object;J)S")
}

#[async_recursion(?Send)]
pub(crate) async fn get_short_volatile(
    thread: Arc<Thread>,
    arguments: Arguments,
) -> Result<Option<Value>> {
    get_reference_type(thread, arguments, Some(BaseType::Short))
}

#[async_recursion(?Send)]
pub(crate) async fn get_uncompressed_object(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.Unsafe.getUncompressedObject(J)Ljava/lang/Object;")
}

#[async_recursion(?Send)]
pub(crate) async fn is_big_endian_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    if cfg!(target_endian = "big") {
        Ok(Some(Value::Int(1)))
    } else {
        Ok(Some(Value::Int(0)))
    }
}

#[async_recursion(?Send)]
pub(crate) async fn load_fence(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
pub(crate) async fn object_field_offset_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    Ok(Some(Value::Long(0)))
}

#[async_recursion(?Send)]
pub(crate) async fn object_field_offset_1(
    thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let value = arguments.pop()?;
    let field_name: String = match value {
        Value::Object(_) => value.try_into()?,
        value => {
            return Err(InvalidOperand {
                expected: "object".to_string(),
                actual: value.to_string(),
            });
        }
    };
    let Some(Reference::Object(class_object)) = arguments.pop_reference()? else {
        return Err(InternalError(
            "objectFieldOffset1: Invalid class reference".to_string(),
        ));
    };
    let class_name: String = class_object.value("name")?.try_into()?;
    let class = thread.class(&class_name).await?;
    let offset = class.field_offset(&field_name)?;
    let offset = i64::try_from(offset)?;
    Ok(Some(Value::Long(offset)))
}

#[async_recursion(?Send)]
pub(crate) async fn page_size(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.Unsafe.pageSize()I")
}

#[async_recursion(?Send)]
pub(crate) async fn park(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.Unsafe.park(ZJ)V")
}

#[async_recursion(?Send)]
pub(crate) async fn put_boolean(
    _thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let x = arguments.pop_int()? != 0;
    let offset = usize::try_from(arguments.pop_long()?)?;
    let Value::Object(ref mut object) = arguments.pop()? else {
        return Err(InternalError("putBoolean: Invalid reference".to_string()));
    };
    let bytes = Reference::from(vec![x; offset]);
    *object = Some(bytes);
    Ok(None)
}

#[async_recursion(?Send)]
pub(crate) async fn put_boolean_volatile(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.Unsafe.putBooleanVolatile(Ljava/lang/Object;JZ)V")
}

#[async_recursion(?Send)]
pub(crate) async fn put_byte(
    _thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let x = i8::try_from(arguments.pop_int()?)?;
    let offset = usize::try_from(arguments.pop_long()?)?;
    let Value::Object(ref mut object) = arguments.pop()? else {
        return Err(InternalError("putByte: Invalid reference".to_string()));
    };
    let bytes = Reference::from(vec![x; offset]);
    *object = Some(bytes);
    Ok(None)
}

#[async_recursion(?Send)]
pub(crate) async fn put_byte_volatile(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.Unsafe.putByteVolatile(Ljava/lang/Object;JB)V")
}

#[async_recursion(?Send)]
pub(crate) async fn put_char(
    _thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    #[expect(clippy::cast_sign_loss)]
    let x = arguments.pop_int()? as u32;
    let Some(x) = char::from_u32(x) else {
        return Err(InternalError("putChar: Invalid character".to_string()));
    };
    let offset = usize::try_from(arguments.pop_long()?)?;
    let Value::Object(ref mut object) = arguments.pop()? else {
        return Err(InternalError("putChar: Invalid reference".to_string()));
    };
    let bytes = Reference::from(vec![x; offset]);
    *object = Some(bytes);
    Ok(None)
}

#[async_recursion(?Send)]
pub(crate) async fn put_char_volatile(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.Unsafe.putCharVolatile(Ljava/lang/Object;JC)V")
}

#[async_recursion(?Send)]
pub(crate) async fn put_double(
    _thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let x = arguments.pop_double()?;
    let offset = usize::try_from(arguments.pop_long()?)?;
    let Value::Object(ref mut object) = arguments.pop()? else {
        return Err(InternalError("putDouble: Invalid reference".to_string()));
    };
    let bytes = Reference::from(vec![x; offset]);
    *object = Some(bytes);
    Ok(None)
}

#[async_recursion(?Send)]
pub(crate) async fn put_double_volatile(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.Unsafe.putDoubleVolatile(Ljava/lang/Object;JD)V")
}

#[async_recursion(?Send)]
pub(crate) async fn put_float(
    _thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let x = arguments.pop_float()?;
    let offset = usize::try_from(arguments.pop_long()?)?;
    let Value::Object(ref mut object) = arguments.pop()? else {
        return Err(InternalError("putFloat: Invalid reference".to_string()));
    };
    let bytes = Reference::from(vec![x; offset]);
    *object = Some(bytes);
    Ok(None)
}

#[async_recursion(?Send)]
pub(crate) async fn put_float_volatile(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.Unsafe.putFloatVolatile(Ljava/lang/Object;JF)V")
}

#[async_recursion(?Send)]
pub(crate) async fn put_int(
    _thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let x = arguments.pop_int()?;
    let offset = usize::try_from(arguments.pop_long()?)?;
    let Value::Object(ref mut object) = arguments.pop()? else {
        return Err(InternalError("putInt: Invalid reference".to_string()));
    };
    let bytes = Reference::from(vec![x; offset]);
    *object = Some(bytes);
    Ok(None)
}

#[async_recursion(?Send)]
pub(crate) async fn put_int_volatile(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.Unsafe.putIntVolatile(Ljava/lang/Object;JI)V")
}

#[async_recursion(?Send)]
pub(crate) async fn put_long(
    _thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let x = arguments.pop_long()?;
    let offset = usize::try_from(arguments.pop_long()?)?;
    let Value::Object(ref mut object) = arguments.pop()? else {
        return Err(InternalError("putlong: Invalid reference".to_string()));
    };
    let bytes = Reference::from(vec![x; offset]);
    *object = Some(bytes);
    Ok(None)
}

#[async_recursion(?Send)]
pub(crate) async fn put_long_volatile(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.Unsafe.putLongVolatile(Ljava/lang/Object;JJ)V")
}

#[async_recursion(?Send)]
pub(crate) async fn put_object(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.Unsafe.putObject(Ljava/lang/Object;JLjava/lang/Object;)V")
}

#[async_recursion(?Send)]
pub(crate) async fn put_object_volatile(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.Unsafe.putObjectVolatile(Ljava/lang/Object;JLjava/lang/Object;)V")
}

#[async_recursion(?Send)]
pub(crate) async fn put_reference(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.Unsafe.putReference(Ljava/lang/Object;JLjava/lang/Object;)V")
}

#[async_recursion(?Send)]
pub(crate) async fn put_reference_volatile(
    _thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let x = arguments.pop()?;
    let offset = arguments.pop_long()?;
    let offset = usize::try_from(offset)?;
    let Some(object) = arguments.pop_reference()? else {
        return Err(InternalError(
            "putReferenceVolatile: Invalid reference".to_string(),
        ));
    };
    match object {
        Reference::Array(_class, array) => {
            let x = x.to_reference()?;
            array.set(offset, x)?;
        }
        Reference::Object(object) => {
            let field_name = object.class().field_name(offset)?;
            object.set_value(&field_name, x)?;
        }
        _ => {
            return Err(InternalError(
                "putReferenceVolatile: Invalid reference".to_string(),
            ));
        }
    }
    Ok(None)
}

#[async_recursion(?Send)]
pub(crate) async fn put_short(
    _thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let x = i16::try_from(arguments.pop_int()?)?;
    let offset = usize::try_from(arguments.pop_long()?)?;
    let Value::Object(ref mut object) = arguments.pop()? else {
        return Err(InternalError("putShort: Invalid reference".to_string()));
    };
    let bytes = Reference::from(vec![x; offset]);
    *object = Some(bytes);
    Ok(None)
}

#[async_recursion(?Send)]
pub(crate) async fn put_short_volatile(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.Unsafe.putShortVolatile(Ljava/lang/Object;JS)V")
}

#[async_recursion(?Send)]
pub(crate) async fn reallocate_memory_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.Unsafe.reallocateMemory0(JJ)J")
}

#[async_recursion(?Send)]
pub(crate) async fn register_natives(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
pub(crate) async fn set_memory_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.Unsafe.setMemory0(Ljava/lang/Object;JJB)V")
}

#[async_recursion(?Send)]
pub(crate) async fn should_be_initialized_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.Unsafe.shouldBeInitialized0(Ljava/lang/Class;)Z")
}

#[async_recursion(?Send)]
pub(crate) async fn static_field_base_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.Unsafe.staticFieldBase0(Ljava/lang/reflect/Field;)Ljava/lang/Object;")
}

#[async_recursion(?Send)]
pub(crate) async fn static_field_offset_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.Unsafe.staticFieldOffset0(Ljava/lang/reflect/Field;)J")
}

#[async_recursion(?Send)]
pub(crate) async fn store_fence(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
pub(crate) async fn throw_exception(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.Unsafe.throwException(Ljava/lang/Throwable;)V")
}

#[async_recursion(?Send)]
pub(crate) async fn unaligned_access_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    Ok(Some(Value::Int(0)))
}

#[async_recursion(?Send)]
pub(crate) async fn unpark(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.Unsafe.unpark(Ljava/lang/Object;)V")
}

#[async_recursion(?Send)]
pub(crate) async fn writeback_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.Unsafe.writeback0(J)V")
}

#[async_recursion(?Send)]
pub(crate) async fn writeback_post_sync_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.Unsafe.writebackPostSync0()V")
}

#[async_recursion(?Send)]
pub(crate) async fn writeback_pre_sync_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.Unsafe.writebackPreSync0()V")
}
