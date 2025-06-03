use std::fs::metadata;
use std::path::{Path, PathBuf};

pub fn list_proto_files(path: &Path) -> Vec<PathBuf> {
    let mut vec = Vec::new();
    _list_files(&mut vec, path);
    vec
}

fn _list_files(vec: &mut Vec<PathBuf>, path: &Path) {
    if metadata(path).unwrap().is_dir() {
        let paths = std::fs::read_dir(path).unwrap();
        for path_result in paths {
            let full_path = path_result.unwrap().path();
            if metadata(&full_path).unwrap().is_dir() {
                _list_files(vec, &full_path);
            } else if !path_is_git(&full_path) {
                vec.push(full_path);
            }
        }
    }
}

fn path_is_git<T: AsRef<Path>>(path: T) -> bool {
    path.as_ref()
        .iter()
        .any(|slice| slice.to_string_lossy().contains(".git"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn no_git_folders() {
        let root_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
        let proto_dir = root_dir
            .parent()
            .expect("root should be the workspace tree")
            .join("proto-types")
            .join("proto");
        let files = list_proto_files(&proto_dir);
        println!("PATH: {proto_dir:?}");
        println!("LEFT: {files:?}");
        let git: Vec<&PathBuf> = files
            .iter()
            .filter(|pathname| pathname.to_string_lossy().contains(".git"))
            .collect();
        println!("RIGHT: {git:?}");

        assert!(git.is_empty());
    }
}
