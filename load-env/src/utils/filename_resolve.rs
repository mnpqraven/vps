use crate::{EnvError, schema::NAME_REGEX};
use regex::Regex;
use std::{fs::read_dir, path::Path};
use tracing::instrument;

pub fn is_legit_filename(filename: &str) -> bool {
    let name_regex = Regex::new(NAME_REGEX).unwrap();
    name_regex.is_match(filename)
}

/// some document
///
/// * `fullpath` - wheter the return the fullpath or just the file name
#[instrument(ret, skip(path), level = "debug")]
pub fn first_legit_file<P: AsRef<Path> + Clone>(
    path: P,
    fullpath: bool,
) -> Result<String, EnvError> {
    let filename_or_path = read_dir(path.clone())
        .map_err(|source| EnvError::Io {
            file_name: Some(path.as_ref().to_string_lossy().to_string()),
            source,
        })?
        .flatten()
        .filter(|dir| is_legit_filename(&dir.file_name().to_string_lossy()))
        .map(|dir| match fullpath {
            true => dir.path().to_string_lossy().into_owned(),
            false => dir.file_name().to_string_lossy().into_owned(),
        })
        .collect::<String>();

    Ok(filename_or_path)
}
