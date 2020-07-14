use std::{env, io};
use std::collections::HashMap;
use std::fs::metadata;
use std::ops::Add;

use bytes::{Buf, Bytes, BytesMut};
use futures::TryStreamExt;
use rusoto_core::Region;
use rusoto_s3::{GetObjectOutput, GetObjectRequest, PutObjectRequest, S3, S3Client};
use serde_json::{json, Value};

use PostDocument::Post::Post;

const POST_BUCKET_NAME: &str = "devblogposts";

fn get_s3_client() -> S3Client {
    return S3Client::new(Region::CaCentral1);
}

fn convertStringToStream(s: String) -> rusoto_s3::StreamingBody {
    return s.into_bytes().into();
}

pub async fn put_file_content(file_content: Post) -> String {
    let mut metadata: HashMap<String, String> = HashMap::new();
    metadata.insert("post_id".to_string(), file_content.id.to_string());
    let stream = convertStringToStream(file_content.content);
    let res_stream = get_s3_client().put_object(
        PutObjectRequest {
            bucket: POST_BUCKET_NAME.to_string(),
            key: file_content.title.to_string().add(".").add(&file_content.file_type),
            body: Option::from(stream),
            metadata: Option::from(metadata),
            acl: Some("public-read".to_string()),
            ..Default::default()
        }
    ).await.unwrap();
    let res_stream_body = res_stream.version_id.unwrap();
    return res_stream_body;
}

pub async fn get_file(file_name: String) -> GetObjectOutput {
    let input = &file_name;
    return get_s3_client().get_object(
        GetObjectRequest {
            bucket: POST_BUCKET_NAME.to_string(),
            key: input.to_string(),
            ..Default::default()
        }
    ).await.unwrap();
}

pub async fn get_file_content(file_name: String) -> String {
    let file = get_file(file_name).await;
    if file.content_length.unwrap() > 0 {
        println!("{}", file.content_length.unwrap())
    } else {
        panic!("{}", "i messed up")
    }
    let stream = file.body.unwrap();
    let body: BytesMut = stream.map_ok(|b| bytes::BytesMut::from(&b[..])).try_concat().await.unwrap();
    let data = body.to_vec();
    let res = String::from_utf8(data).unwrap();
    return res;
}