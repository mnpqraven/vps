use strum::AsRefStr;
use strum::Display;
use strum::IntoStaticStr;

#[derive(AsRefStr, IntoStaticStr, Display, Debug)]
#[strum(prefix = "/", serialize_all = "snake_case")]
pub enum RouterKey {
    #[strum(to_string = "")]
    Root,
    Nas,
    RpcServer,
    Database,
    #[strum(to_string = "database/fhs")]
    DatabaseFhs,
    #[strum(to_string = "database/health")]
    DatabaseHealth,
    #[strum(to_string = "database/tables")]
    DatabaseTables,
    #[strum(to_string = "database/tables/blog")]
    DatabaseTablesBlog,
    #[strum(to_string = "database/tables/blog/{0}")]
    DatabaseTablesBlogDetail(String),
    #[strum(to_string = "database/tables/blog/create")]
    DatabaseTablesBlogCreate,
    #[strum(to_string = "database/tables/blog_tag")]
    DatabaseTablesBlogTag,
    #[strum(to_string = "database/tables/blog_tag/detail/{0}")]
    DatabaseTablesBlogTagDetail(String),
    #[strum(to_string = "database/tables/blog_tag/create")]
    DatabaseTablesBlogTagCreate,
}

#[cfg(test)]
mod tests {
    use super::RouterKey;

    #[test]
    fn router_key_enum_to_string_impl() {
        let left = RouterKey::Root;
        assert_eq!(left.to_string(), String::from("/"));

        let left = RouterKey::RpcServer;
        assert_eq!(left.to_string(), String::from("/rpc_server"));

        let left = RouterKey::DatabaseFhs;
        assert_eq!(left.to_string(), String::from("/database/fhs"));

        let left = RouterKey::DatabaseTablesBlogTagCreate;
        assert_eq!(
            left.to_string(),
            String::from("/database/tables/blog_tag/create")
        );
    }
}
