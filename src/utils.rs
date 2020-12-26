use crate::errors::{OssClientError};

pub fn check_bucket_name(bucket_name: String) -> Result<(), OssClientError> {
    let name_len = bucket_name.len();
    if name_len < 3 || name_len > 63 {
        return Err(OssClientError::BucketNameLengthError {bucket_name, name_len})
    }

    for v in bucket_name.chars() {
        if !(('a' <= v && v <= 'z') || ('0' <= v && v <= '9') || v == '-') {
            return Err(OssClientError::BucketNameIsIllegal {bucket_name})
        }
    }

    let mut bucket_char = bucket_name.chars();
    if bucket_char.next() == Some('-') || bucket_char.last() == Some('-') {
        return Err(OssClientError::BucketNameIsIllegal {bucket_name})
    }
    Ok(())
}
