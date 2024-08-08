use crate::class_path_entry::directory::Directory;
use crate::class_path_entry::jar::Jar;
use crate::Result;
use ristretto_classfile::ClassFile;
use std::fmt::Debug;
use std::path::PathBuf;
use std::sync::Arc;
use tracing::instrument;

/// Represents a class path entry.
#[derive(Debug, PartialEq)]
pub enum ClassPathEntry {
    Directory(Directory),
    Jar(Jar),
    #[cfg(feature = "url")]
    Url(crate::class_path_entry::url::Url),
}

/// Default implementation for `ClassPathEntry`.
impl Default for ClassPathEntry {
    /// Returns a class path entry with a default directory.
    fn default() -> Self {
        ClassPathEntry::Directory(Directory::default())
    }
}

/// Implementation for `ClassPathEntry`.
impl ClassPathEntry {
    /// Create a new class path entry.
    pub fn new<S: AsRef<str>>(path: S) -> Self {
        let path = path.as_ref();
        #[cfg(feature = "url")]
        if path.starts_with("https://") || path.starts_with("http://") {
            return ClassPathEntry::Url(crate::class_path_entry::url::Url::new(path));
        }

        if PathBuf::from(path).is_file() {
            ClassPathEntry::Jar(Jar::new(path))
        } else {
            ClassPathEntry::Directory(Directory::new(path))
        }
    }

    /// Get the name of the class path entry.
    pub fn name(&self) -> String {
        match self {
            ClassPathEntry::Directory(directory) => directory.name(),
            ClassPathEntry::Jar(jar) => jar.name(),
            #[cfg(feature = "url")]
            ClassPathEntry::Url(url) => url.name(),
        }
    }

    /// Read a class from the class path entry.
    ///
    /// # Errors
    /// if the class file cannot be read.
    #[instrument(level = "trace", fields(name = ?name.as_ref()), skip(self))]
    pub async fn read_class<S: AsRef<str>>(&self, name: S) -> Result<Arc<ClassFile>> {
        match self {
            ClassPathEntry::Directory(directory) => directory.read_class(name).await,
            ClassPathEntry::Jar(jar) => jar.read_class(name).await,
            #[cfg(feature = "url")]
            ClassPathEntry::Url(url) => url.read_class(name).await,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let class_path_entry = ClassPathEntry::default();
        assert_eq!(class_path_entry.name(), ".");
    }

    //
    // Directory Tests
    //

    #[test]
    fn test_new_directory() {
        let cargo_manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let classes_directory = cargo_manifest.join("../classes");
        let class_path_entry = ClassPathEntry::new(classes_directory.to_string_lossy());

        assert!(matches!(class_path_entry, ClassPathEntry::Directory(_)));
        assert_eq!(class_path_entry.name(), classes_directory.to_string_lossy());
    }

    #[tokio::test]
    async fn test_read_class_directory() -> Result<()> {
        let cargo_manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let classes_directory = cargo_manifest.join("../classes");
        let class_path_entry = ClassPathEntry::new(classes_directory.to_string_lossy());
        let class_file = class_path_entry.read_class("HelloWorld").await?;

        assert!(matches!(class_path_entry, ClassPathEntry::Directory(_)));
        assert_eq!(class_path_entry.name(), classes_directory.to_string_lossy());
        assert_eq!("HelloWorld", class_file.class_name()?);
        Ok(())
    }

    //
    // Jar Tests
    //

    #[test]
    fn test_new_jar() {
        let cargo_manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let classes_jar = cargo_manifest.join("../classes/classes.jar");
        let class_path_entry = ClassPathEntry::new(classes_jar.to_string_lossy());

        assert!(matches!(class_path_entry, ClassPathEntry::Jar(_)));
        assert_eq!(class_path_entry.name(), classes_jar.to_string_lossy());
    }

    #[tokio::test]
    async fn test_read_class_jar() -> Result<()> {
        let cargo_manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let classes_jar = cargo_manifest.join("../classes/classes.jar");
        let class_path_entry = ClassPathEntry::new(classes_jar.to_string_lossy());
        let class_file = class_path_entry.read_class("HelloWorld").await?;

        assert!(matches!(class_path_entry, ClassPathEntry::Jar(_)));
        assert_eq!(class_path_entry.name(), classes_jar.to_string_lossy());
        assert_eq!("HelloWorld", class_file.class_name()?);
        Ok(())
    }

    //
    // Url Tests
    //

    #[cfg(feature = "url")]
    #[test]
    fn test_new_url() {
        let url = "https://repo1.maven.org/maven2/org/springframework/boot/spring-boot/3.3.0/spring-boot-3.3.0.jar";
        let class_path_entry = ClassPathEntry::new(url);

        assert!(matches!(class_path_entry, ClassPathEntry::Url(_)));
        assert_eq!(class_path_entry.name(), url);
    }

    #[cfg(feature = "url")]
    #[tokio::test]
    async fn test_read_class_url() -> Result<()> {
        let url = "https://repo1.maven.org/maven2/org/springframework/boot/spring-boot/3.3.0/spring-boot-3.3.0.jar";
        let class_path_entry = ClassPathEntry::new(url);
        let class_file = class_path_entry
            .read_class("org.springframework.boot.SpringApplication")
            .await?;

        assert!(matches!(class_path_entry, ClassPathEntry::Url(_)));
        assert_eq!(class_path_entry.name(), url);
        assert_eq!(
            "org/springframework/boot/SpringApplication",
            class_file.class_name()?
        );
        Ok(())
    }
}