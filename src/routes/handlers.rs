use std::thread;
use async_std::io::{ReadExt, WriteExt};
use async_std::net::TcpStream;

use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use async_std::fs::File;
use async_std::prelude::*;
use crate::models::Item;

pub async fn get_items() -> impl Responder {
    println!("Request come");
    let items = vec![
        Item { id: 1, name: "Item 1 from Rust server".to_string() },
        Item { id: 2, name: "Item 2 from Rust server".to_string() },
    ];
    HttpResponse::Ok().json(items)
}

pub async fn get_item(path: web::Path<u32>) -> impl Responder {
    let item = Item { id: path.into_inner(), name: "Item".to_string() };
    HttpResponse::Ok().json(item)
}

pub async fn create_item(item: web::Json<Item>) -> impl Responder {
    HttpResponse::Created().json(item.into_inner())
}

pub async fn update_item(path: web::Path<u32>, item: web::Json<Item>) -> impl Responder {
    let id = path.into_inner();
    let updated_item = Item { id, name: item.name.clone() };
    HttpResponse::Ok().json(updated_item)
}

pub async fn delete_item(path: web::Path<u32>) -> impl Responder {
    let id = path.into_inner();
    HttpResponse::Ok().json(format!("Deleted item with id: {}", id))
}

pub async fn read_file() -> impl Responder {
    let path_loc: &str = r"";
    let mut file = File::open(path_loc).await.unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).await.unwrap();
    HttpResponse::Ok().body(contents)
}

pub async fn get_file_by_name(path: web::Path<String>) -> impl Responder {
    let filename = path.into_inner();
    let filepath = format!("./{}", filename);
    match File::open(&filepath).await {
        Ok(mut file) => {
            let mut contents = String::new();
            if let Err(_) = file.read_to_string(&mut contents).await {
                return HttpResponse::InternalServerError().body("Error reading file");
            }
            HttpResponse::Ok().body(contents)
        }
        Err(_) => HttpResponse::NotFound().body("File not found"),
    }
}

#[get("/print/{areaId}/{imgId}")]
pub async fn print_ids(path: web::Path<(u32, u32)>) -> impl Responder {
    let (area_id, img_id) = path.into_inner();
    println!("Received areaId: {}, imgId: {}", area_id, img_id);
    let mut content = format!("Received areaId: {}, imgId: {}", area_id, img_id);
    HttpResponse::Ok().body(content)
}


pub async fn fetch_from_java() -> impl Responder {
    // Connect to the Java server asynchronously using async-std
    match TcpStream::connect("127.0.0.1:7878").await {
        Ok(mut stream) => {
            // Optionally send a message to the Java server
            let request_message = b"Hello, Java Server!\n";
            if let Err(e) = stream.write_all(request_message).await {
                return HttpResponse::InternalServerError().body(format!("Failed to send request: {}", e));
            }

            // Read the response from the Java server
            let mut buffer = vec![0; 512];
            match stream.read(&mut buffer).await {
                Ok(bytes_read) => {
                    let response = String::from_utf8_lossy(&buffer[..bytes_read]);
                    println!("{}", response);

                    HttpResponse::Ok().body(response.to_string())
                }
                Err(e) => HttpResponse::InternalServerError().body(format!("Failed to read response: {}", e)),
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to connect: {}", e)),
    }
}


// THREADS

fn add(a: i32, b: i32, c: i32) -> i32 {
    a + b + c
}

fn subtract(a: i32, b: i32, c: i32) -> i32 {
    a - b - c
}

fn multiply(a: i32, b: i32, c: i32) -> i32 {
    a * b * c
}


pub async fn compute_operations(a: i32, b: i32, c: i32) -> impl Responder {

    let handle_add = {
        let (a, b, c) = (a, b, c);
        thread::spawn(move || add(a, b, c))
    };

    let handle_subtract = {
        let (a, b, c) = (a, b, c);
        thread::spawn(move || subtract(a, b, c))
    };

    let handle_multiplication = {
        let (a, b, c) = (a, b, c);
        thread::spawn(move || multiply(a, b, c))
    };

    let result_add = handle_add.join().unwrap();
    let result_subtract = handle_subtract.join().unwrap();
    let result_multiply = handle_multiplication.join().unwrap();

    let final_result = result_add + result_subtract + result_multiply;

    let content = format!("final_result {}", final_result);
    HttpResponse::Ok().body(content)
}