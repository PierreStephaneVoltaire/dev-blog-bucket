
use std::{env, io};
use bytes::{Bytes, BytesMut, Buf};
use serde_json::{json, Value};
use rusoto_s3::{S3Client, S3, GetObjectRequest, GetObjectOutput};
use rusoto_core::Region;
use futures::TryStreamExt;






const POST_BUCKET_NAME: &str = "devblogposts";

fn get_s3_client()->S3Client{
 return S3Client::new(Region::CaCentral1);

}

pub async fn get_file(file_name:String) -> GetObjectOutput {
 let input=&file_name;
 return get_s3_client().get_object(
  GetObjectRequest{
   bucket: input.to_string(),
   key: (&file_name).parse().unwrap(),
   ..Default::default()
  }
 ).await.unwrap();

}

 pub async fn get_file_content(file_name:String) -> String {
  let file=get_file(file_name).await;
  let stream=file.body.unwrap();
  let body:BytesMut = stream.map_ok(|b| bytes::BytesMut::from(&b[..])).try_concat().await.unwrap();
  let data = body.to_vec();
  let res=String::from_utf8(data).unwrap();
  return res;
}