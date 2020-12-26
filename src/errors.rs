use thiserror::Error;
use std::fmt::Display;

#[derive(Error, Debug)]
pub enum OssClientError {
    #[error("bucket name '{bucket_name}' len is between [3-63],now is {name_len}")]
    BucketNameLengthError {bucket_name: String, name_len: usize},
    #[error("bucket name '{bucket_name}' can only include lowercase letters, numbers, and -")]
    BucketNameIsIllegal{bucket_name: String},
    #[error("Unknown error")]
    Unknown,
}



pub fn display_err<T: Display>(value: T) -> String {
    return value.to_string();
}
