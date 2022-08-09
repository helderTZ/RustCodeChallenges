use std::path::Path;
use std::fs;

trait FileMetadata {
    fn exists(&self) -> bool;
    fn is_writable(&self) -> bool;
    fn is_readable(&self) -> bool;
}

impl FileMetadata for Path {
    fn exists(&self) -> bool {
        // fs::metadata(self).is_ok()
        self.exists()
    }

    fn is_writable(&self) -> bool {
        // if let Ok(m) = fs::metadata(self) {
        //     !m.permissions().readonly()
        // } else {
        //     false
        // }

        // another way of doing it
        fs::metadata(self)
            .map(|m| {
                !m.permissions().readonly()
            })
            .unwrap_or(false)
    }

    fn is_readable(&self) -> bool {
        fs::File::open(&self).is_ok()
        
    }
}

fn main() {
    //
}

#[test]
fn exists() {
    use std::fs;
    use tempfile;

    let f = tempfile::NamedTempFile::new().unwrap();
    assert_eq!(f.path().exists(), true);

    fs::remove_file(f.path()).unwrap();
    assert_eq!(f.path().exists(), false);

}

#[test]
fn writable() {
    use std::fs;
    use tempfile;

    let f = tempfile::NamedTempFile::new().unwrap();
    assert!(f.path().is_writable());

    fs::remove_file(f.path()).unwrap();
}

#[test]
fn read_only() {
    use std::fs;
    use tempfile;

    let f = tempfile::NamedTempFile::new().unwrap();
    let mut perms = fs::metadata(f.path()).unwrap().permissions();
    perms.set_readonly(true);
    fs::set_permissions(f.path(), perms).unwrap();
    assert_eq!(f.path().is_writable(), false);

    fs::remove_file(f.path()).unwrap();
}