use crate::blog::{meta::BlogMeta, tag::BlogTag};

pub struct Blog {
    pub meta: BlogMeta,
    pub tags: Vec<BlogTag>,
    pub content: String,
}
