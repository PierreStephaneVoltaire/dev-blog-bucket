#[macro_use]
extern crate actix_web;
use serde::Deserialize;

use std::{env, io};
use actix_utils::mpsc;
use actix_web::http::{header, Method, StatusCode};
use actix_web::{
    error, guard, middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer,
    Result,
};
use bytes::{Bytes, BytesMut, Buf};
use serde_json::{json, Value};
use rusoto_s3::{S3Client, S3, GetObjectRequest};
use rusoto_core::Region;
use futures::TryStreamExt;
use bucket::S3::get_file_content;


#[get("/{name}")]
async fn get_doc_content(doc_name: web::Path<(String)>) -> Result<HttpResponse> {
    let document_name=doc_name.into_inner();
   let file_content=get_file_content(document_name);
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("application/json; charset=utf-8")
        .body(file_content.await))
}


#[post("/")]
async fn create_doc() -> Result<HttpResponse> {
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("application/json; charset=utf-8")
        .body(""))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .service(get_doc_content)
            .service(create_doc)

            .wrap(middleware::Logger::default())
    }).bind("127.0.0.1:8000")?
        .run()
        .await
}