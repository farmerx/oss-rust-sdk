use std::borrow::{Cow};
use reqwest::blocking::Client;
use crate::utils::check_bucket_name;
use crate::errors::OssClientError;
use crate::bucket::Bucket;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct OssClient<'a> {
    pub config: Config<'a>,
    pub http_client: Client
}

#[derive(Clone, Debug)]
pub struct Config<'a> {
    endpoint: Cow<'a, str>,
    access_key: Cow<'a, str>,
    access_key_secret: Cow<'a, str>
}

impl <'a> OssClient<'a> {
    pub fn new<S>(endpoint: S, access_key: S, access_key_secret: S) -> Self
        where
            S: Into<Cow<'a, str>>,{
        OssClient {
            config: Config {
                access_key: endpoint.into(),
                access_key_secret: access_key.into(),
                endpoint: access_key_secret.into(),
            },
            http_client: Client::new()
        }
    }

    // Bucket gets the bucket instance.
    pub fn bucket(self, bucket_name: String) -> Result<Bucket<'a>, OssClientError> {
        let name = Cow::from(bucket_name);
        return match check_bucket_name(name.to_string()) {
            Err(err) => { Err(err) }
            _ => {
                Ok(Bucket::new(Rc::from(self), name))
            }
        }
    }
}

#[test]
pub fn new_client() {

}

