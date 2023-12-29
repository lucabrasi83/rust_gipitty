use actix_cors::Cors;
use actix_web::{http::header, web, App, HttpResponse, HttpServer, Responder};
use async_trait::async_trait;
use reqwest::Client as HttpClient;
use serde::{Deserialize, Serialize};

use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::sync::Mutex;

fn main() {
    println!("Hello, world!");
}
