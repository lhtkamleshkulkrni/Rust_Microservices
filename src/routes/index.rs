#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused)]
#![allow(non_snake_case)]

use actix_web::Responder;

use actix_web::{delete, get, post, put, web, web::Json, HttpRequest, HttpResponse};

use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};

use mongodb::Client;

use serde::{Deserialize, Serialize};

use std::path::Path;

use thiserror::Error;

use user_controller::filter_model::Filter;

// use user_controller::model::Claims;
use jwt_controller::model::Claims;

use user_controller::model::User;
use uuid::Uuid;

#[path = "../app/modules/user/index.rs"]
mod user_controller;

#[path = "../app/services/index.rs"]
mod aws_controller;

#[path = "../middleware/index.rs"]
mod jwt_controller;

use aws_controller::model::Download_File;
use aws_controller::model::Upload_File;
#[path = "../app/models/user.rs"]
mod model;

const JWT_SECRET: &[u8] = b"secret";
// routes
#[get("/")]
pub async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Rust microservice alive!")
}
#[get("/get-user/{id}")]
pub async fn get_user(
    client: web::Data<Client>,
    id: web::Path<String>,
    req: HttpRequest,
) -> HttpResponse {
    let header_token = req.headers()
    .get("authorization")
    .unwrap()
    .to_str()
    .unwrap()
    .to_owned();

    // let token = header_token.to_string();
    let token = header_token[7..header_token.len()].to_string();
    let res = jwt_controller::check_token(token).await;
    if res == true {
        let user_details = user_controller::get_user(client, id).await;
        match user_details {
            Ok(user) => HttpResponse::Ok().json(user),
            Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
        }
    } else {
        HttpResponse::InternalServerError().body("authorization failed")
    }
    // let header_token = req.headers()
    // .get("authorization")
    // .unwrap()
    // .to_str()
    // .unwrap()
    // .to_owned();
    //     println!("token ---> {:?}",header_token);
    // // let token = header_token.to_string();
    // let token = header_token[7..header_token.len()].to_string();
    // let res = jwt_controller::check_token(token).await;

    // if res == true {
    //     HttpResponse::Ok().body("token is authorize.........")
    // }else {HttpResponse::Ok().body("this is not authorize........!!!")}
}

#[post("/create-user")]
pub async fn create_user(client: web::Data<Client>, req: Json<User>) -> HttpResponse {
    let uid = Uuid::new_v4();
    let request_data = User {
        id: uid.to_string(),
        first_name: req.first_name.to_owned(),
        last_name: req.last_name.to_owned(),
        username: req.username.to_owned(),
        email: req.email.to_owned(),
    };
    let response = user_controller::create_user(client, request_data).await;
    match response {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
#[put("/update-user")]
pub async fn update_user(client: web::Data<Client>, req: Json<User>) -> HttpResponse {
    let request_data = User {
        id: req.id.to_owned(),
        first_name: req.first_name.to_owned(),
        last_name: req.last_name.to_owned(),
        username: req.username.to_owned(),
        email: req.email.to_owned(),
    };
    let response =
        user_controller::update_user(client, req.id.to_owned(), request_data, req.id.to_owned())
            .await;
    match response {
        Ok(updated_user) => HttpResponse::Ok().json(updated_user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
#[get("/get-all-users")]
pub async fn get_all_users(client: web::Data<Client>) -> HttpResponse {
    let response = user_controller::get_all_users(client).await;
    match response {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
#[delete("/delete-user/{id}")]
pub async fn delete_user(client: web::Data<Client>, id: web::Path<String>) -> HttpResponse {
    let _id = id.into_inner();
    let response = user_controller::delete_user(client, _id).await;
    match response {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
// JWT APIs ---------------------
#[post("/create-jwt-token")]
pub async fn create_jwt_token(req_data: Json<Claims>) -> HttpResponse {
    let request_data = req_data.into_inner();
    // let token_detail = user_controller::create_jwt_token(request_data).await;
    let token_detail = jwt_controller::create_jwt_token(request_data).await;
    match token_detail {
        Ok(token) => HttpResponse::Ok().json(token),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
#[get("/check-token")]
pub async fn check_token(clinent: web::Data<Client> , req: HttpRequest) -> HttpResponse {
    let header_token = req.headers()
    .get("authorization")
    .unwrap()
    .to_str()
    .unwrap()
    .to_owned();

    let token = header_token.to_string();
    let res = jwt_controller::check_token(header_token).await;

    if res == true {
        HttpResponse::Ok().body("token is authorize.........")
    }else {HttpResponse::Ok().body("this is not authorize........!!!")}
}
//search user api 
#[post("/search_user")]
pub async fn search_users(client: web::Data<Client>, new_filter: Json<Filter>, req: HttpRequest) -> HttpResponse {
    let header_token = req.headers()
    .get("authorization")
    .unwrap()
    .to_str()
    .unwrap()
    .to_owned();

    let token = header_token.to_string();
    let res = jwt_controller::check_token(token).await;
    if res == true {
    let data = Filter {
        filter_field: new_filter.filter_field.to_owned(),
        filter_key: new_filter.filter_key.to_owned(),
    };
    let user_details = user_controller::get_search_user(client, data).await;
    match user_details {
        Ok(user_details) => HttpResponse::Ok().json(user_details),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}else {HttpResponse::Ok().body("this is not authorize........!!!")}
}

//aws s3 api's
// upload file
#[post("/upload-file")]

pub async fn upload_file(path: Json<Upload_File>) -> impl Responder {
    let x = path.file_path.to_string();

    let file_path = Path::new(&x);

    let file_detail = aws_controller::file_upload(file_path).await;

    match file_detail {
        Ok(data) => {
            if data {
                HttpResponse::Ok().body("file upload done .......")
            } else {
                HttpResponse::Ok().body("file upload fail ...")
            }
        }

        Err(err) => {
            HttpResponse::InternalServerError().body("file not upload due to same error ....")
        }
    }
}

#[get("/download_file/{file_name}")]
pub async fn download_file(file_name:web::Path<String>) -> HttpResponse {
    let path = file_name.as_str();
    let file_detail = aws_controller::file_downloaded(path).await; 
    match file_detail {
        Ok(data) => {
            if data {
                HttpResponse::Ok().body("file download done .......")
            }else{HttpResponse::Ok().body("file download fail ...")}
        },
        Err(err) => HttpResponse::InternalServerError().body(format!("file not download due to same error .... {:?}",err))
    }
}
