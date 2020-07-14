#[macro_use]
extern crate actix_web;

use std::{thread, time};
use std::{env, io};
use std::borrow::Borrow;
use std::sync::mpsc;

use actix_rt::System;
use actix_web::{
    App, error, Error, guard, HttpRequest, HttpResponse, HttpServer, middleware, Result,
    web,
};
use actix_web::dev::Server;
use actix_web::http::{header, Method, StatusCode};
use bucket::S3::{get_file_content, put_file_content};
use bytes::{Buf, Bytes, BytesMut};
use dotenv;
use futures::TryStreamExt;
use rusoto_core::Region;
use rusoto_s3::{GetObjectRequest, S3, S3Client};
use rusoto_sqs::{CreateQueueRequest, DeleteMessageRequest, ReceiveMessageRequest, Sqs, SqsClient};
use serde::Deserialize;
use serde_json::{json, Value};

use PostDocument::Post::Post;

#[get("/{name}")]
async fn get_doc_content(doc_name: web::Path<(String)>) -> Result<HttpResponse> {
    let document_name = doc_name.into_inner();
    let file_content = get_file_content(document_name);
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("application/json; charset=utf-8")
        .body(file_content.await))
}


#[post("/")]
async fn create_doc(document_content: web::Json<Post>,
) -> Result<String> {
    let content = document_content.0;
    Ok(put_file_content(content).await)
}

fn run_app(tx: mpsc::Sender<Server>) -> std::io::Result<()> {
    let mut sys = System::new("server");

    // srv is server controller type, `dev::Server`
    let srv =
        HttpServer::new(|| {
            App::new()
                .service(get_doc_content)
                .service(create_doc)

                .wrap(middleware::Logger::default())
        }).bind("127.0.0.1:8000")?
            .run();


    // send server controller to main thread
    let _ = tx.send(srv.clone());

    // run future
    sys.block_on(srv)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let qpath = env::var("qpath").unwrap();
    env_logger::init();

    let (tx, rx) = mpsc::channel();

    println!("START SERVER");
    thread::spawn(move || {
        let _ = run_app(tx);
    });


    let sqs = SqsClient::new(
        Region::CaCentral1
    );
    let mut req = ReceiveMessageRequest::default();
    req.queue_url = qpath.to_string();
    loop {
        let response = sqs.receive_message(req.clone()).await.unwrap();
        let messages = response.messages.unwrap_or_default();
        if !messages.is_empty() {
            for (i, x) in messages.iter().enumerate() {
                let body = x.body.clone();
                println!("{}", body.unwrap());
                let mut del_req = DeleteMessageRequest::default();
                del_req.queue_url = qpath.to_string();
                del_req.receipt_handle = x.receipt_handle.clone().unwrap();
                println!("{}", x.message_id.clone().unwrap());
                sqs.delete_message(del_req);
            }
        }
    }
}