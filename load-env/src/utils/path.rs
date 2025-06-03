use std::path::PathBuf;
use tracing::instrument;

/// priority: see [`EnvSchema`]
#[instrument(ret, level = "debug")]
pub fn get_first_valid_dir() -> Option<PathBuf> {
    match (cargo_dir(), user_config_dir()) {
        (Some(cargo), _) => Some(cargo),
        (_, Some(user)) => Some(user),
        _ => None,
    }
}

/// ./vps
#[instrument(ret, level = "debug")]
pub fn cargo_dir() -> Option<PathBuf> {
    if let Ok(cargo_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        let crate_path = PathBuf::from(cargo_dir);
        // FIXME: unwrap
        let crate_path = crate_path.parent().unwrap().to_path_buf();
        Some(crate_path)
    } else {
        None
    }
}

/// /home/<username>/.config/vps/config.toml
#[instrument(ret, level = "debug")]
pub fn user_config_dir() -> Option<PathBuf> {
    if let Ok(username) = std::env::var("USER") {
        let path = PathBuf::from(format!("/home/{username}/.config/vps"));
        return Some(path);
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::utils::path::{cargo_dir, user_config_dir};

    #[test]
    fn correct_user_dir() {
        assert_eq!(cargo_dir(), Some("/home/othi/Repos/private/vps".into()));
    }

    #[test]
    fn correct_cargo_dir() {
        assert_eq!(user_config_dir(), Some("/home/othi/.config/vps".into()));
    }
}
