use sqlx::{Pool, Postgres};

pub type Db = Pool<Postgres>;

#[derive(Debug)]
pub struct Pagination(pub i64, pub i64);

impl Pagination {
    pub fn index(&self) -> i64 {
        self.0
    }

    pub fn size(&self) -> i64 {
        self.1
    }

    pub fn offset(&self) -> i64 {
        self.0 * self.1
    }
}

impl Default for Pagination {
    fn default() -> Self {
        Self(0, 10)
    }
}
