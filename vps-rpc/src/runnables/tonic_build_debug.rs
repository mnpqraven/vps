use std::{
    env,
    fs::metadata,
    path::{Path, PathBuf},
};

fn main() {
    let root_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let proto_dir = root_dir.join("proto");
    let files: Vec<PathBuf> = list_files(&proto_dir)
        .into_iter()
        .map(|file| file.strip_prefix(root_dir.clone()).unwrap().to_path_buf())
        .collect();
    dbg!(files);
}

fn list_files(path: &Path) -> Vec<PathBuf> {
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
            } else {
                vec.push(full_path);
            }
        }
    }
}
