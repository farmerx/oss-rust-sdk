use crate::client::OssClient;
use std::borrow::{Cow};
use std::rc::Rc;

// Bucket implements the operations of object.
pub struct Bucket<'a> {
    pub client: Rc<OssClient<'a>>,
    pub bucket_name: Cow<'a, str>,
}

impl <'a>Bucket<'a> {
    pub fn new<S>(client: Rc<OssClient<'a>>, bucket_name: S) -> Self
        where
            S: Into<Cow<'a, str>>,
    {
        Self {
            bucket_name: bucket_name.into(),
            client
        }
    }
}