use std::fs::OpenOptions;
use std::io::ErrorKind;
use std::path;

trait FileMetadata {
    fn exists(&self) -> bool;

    fn is_writeable(&self) -> bool;

    fn is_readable(&self) -> bool;
}

impl FileMetadata for path::Path {
    fn exists(&self) -> bool {
        match OpenOptions::new().read(true).open(self) {
            Ok(_) => true,
            Err(e) => {
                !matches!(e.kind(), ErrorKind::NotFound)
            }
        }
    }

    fn is_writeable(&self) -> bool {
        OpenOptions::new().write(true).open(self).is_ok()
    }

    fn is_readable(&self) -> bool {
        OpenOptions::new().read(true).open(self).is_ok()
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_exists() {
        let f = tempfile::NamedTempFile::new().unwrap();
        assert!(f.path().exists());

        fs::remove_file(f.path()).unwrap();
    }

    #[test]
    fn writeable() {
        let f = tempfile::NamedTempFile::new().unwrap();
        assert!(f.path().is_writeable());

        fs::remove_file(f.path()).unwrap();
    }

    #[test]
    fn read_only() {
        let f = tempfile::NamedTempFile::new().unwrap();
        let mut perms = fs::metadata(f.path()).unwrap().permissions();
        perms.set_readonly(true);
        fs::set_permissions(f.path(), perms).unwrap();
        assert!(!f.path().is_writeable());

        fs::remove_file(f.path()).unwrap();
    }
}
