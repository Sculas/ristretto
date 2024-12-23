use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `com.sun.imageio.plugins.jpeg.JPEGImageWriter`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "com/sun/imageio/plugins/jpeg/JPEGImageWriter";
    registry.register(class_name, "abortWrite", "(J)V", abort_write);
    registry.register(class_name, "disposeWriter", "(J)V", dispose_writer);
    registry.register(
        class_name,
        "initJPEGImageWriter",
        "()J",
        init_jpeg_image_writer,
    );
    registry.register(
        class_name,
        "initWriterIDs",
        "(Ljava/lang/Class;Ljava/lang/Class;)V",
        init_writer_ids,
    );
    registry.register(class_name, "resetWriter", "(J)V", reset_writer);
    registry.register(class_name, "setDest", "(J)V", set_dest);
    registry.register(class_name, "writeImage", "(J[BIII[IIIIII[Ljavax/imageio/plugins/jpeg/JPEGQTable;Z[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;ZZZI[I[I[I[I[IZI)Z", write_image);
    registry.register(class_name, "writeTables", "(J[Ljavax/imageio/plugins/jpeg/JPEGQTable;[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;)V", write_tables);
}

#[async_recursion(?Send)]
async fn abort_write(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("com.sun.imageio.plugins.jpeg.JPEGImageWriter.abortWrite(J)V")
}

#[async_recursion(?Send)]
async fn dispose_writer(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("com.sun.imageio.plugins.jpeg.JPEGImageWriter.disposeWriter(J)V")
}

#[async_recursion(?Send)]
async fn init_jpeg_image_writer(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("com.sun.imageio.plugins.jpeg.JPEGImageWriter.initJPEGImageWriter()J")
}

#[async_recursion(?Send)]
async fn init_writer_ids(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("com.sun.imageio.plugins.jpeg.JPEGImageWriter.initWriterIDs(Ljava/lang/Class;Ljava/lang/Class;)V")
}

#[async_recursion(?Send)]
async fn reset_writer(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("com.sun.imageio.plugins.jpeg.JPEGImageWriter.resetWriter(J)V")
}

#[async_recursion(?Send)]
async fn set_dest(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("com.sun.imageio.plugins.jpeg.JPEGImageWriter.setDest(J)V")
}

#[async_recursion(?Send)]
async fn write_image(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("com.sun.imageio.plugins.jpeg.JPEGImageWriter.writeImage(J[BIII[IIIIII[Ljavax/imageio/plugins/jpeg/JPEGQTable;Z[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;ZZZI[I[I[I[I[IZI)Z")
}

#[async_recursion(?Send)]
async fn write_tables(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("com.sun.imageio.plugins.jpeg.JPEGImageWriter.writeTables(J[Ljavax/imageio/plugins/jpeg/JPEGQTable;[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;)V")
}
