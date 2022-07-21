#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused)]
#![allow(non_camel_case_types)]
use actix_web::web;
use chrono::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use mongodb::bson::doc;
use mongodb::bson::extjson::de::Error;
use mongodb::results::{DeleteResult, InsertOneResult};
use mongodb::{Client, Collection};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[path = "../app/constants/index.rs"]
mod constants;

#[path = "../app/models/user.rs"]
pub(crate) mod model;

use model::Claims;
use model::User;
#[derive(Error, Debug)]
pub enum Error_JWT {
    #[error("wrong credentials")]
    WrongCredentialsError,
    #[error("jwt token not valid")]
    JWTTokenError,
    #[error("jwt token creation error")]
    JWTTokenCreationError,
    #[error("no auth header")]
    NoAuthHeaderError,
    #[error("invalid auth header")]
    InvalidAuthHeaderError,
    #[error("no permission")]
    NoPermissionError,
}
type Result_JWT<T> = std::result::Result<T, Error_JWT>;
const JWT_SECRET: &[u8] = b"secret";
#[derive(Debug, Deserialize, Serialize)]
pub struct Info {
    username: String,
    pass: String,
    exp: u64,
}

// JWT Implementation ---------------------
//create jwt token
pub async fn create_jwt_token(request_data: Claims) -> Result_JWT<String> {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::minutes(30))
        .expect("valid timestamp")
        .timestamp();
    let claims = Info {
        username: request_data.username.to_owned(),
        pass: request_data.password.to_string(),
        exp: expiration as u64,
    };
    let header = Header::new(Algorithm::HS512);
    let token = encode(&header, &claims, &EncodingKey::from_secret(JWT_SECRET))
        .map_err(|_| Error_JWT::JWTTokenCreationError);
    return token;
}

//check token
pub async fn check_token(request_data: String) ->bool {
    let client = Client::with_uri_str(constants::DB_URL)
        .await
        .expect("failed to connect");
        
    let t= decode::<Info>(&request_data, &DecodingKey::from_secret(JWT_SECRET), &Validation::new(Algorithm::HS512));
    match t {
        Err(err) => {
            println!("error ---> {:?}",err);
            return false;
        },
        // Ok(data) => println!("token --> {:?}",data.claims.id),
        Ok(data) => {
            let now_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
            if data.claims.exp > now_time {
                let collection: Collection<User> = client
                    .database(constants::DB_NAME)
                    .collection(constants::USER_COLLECTION);
                let user_detail = collection
                    .find_one(doc! {"email": data.claims.username}, None)
                    .await
                    .ok()
                    .expect("Error getting user's detail");

                if user_detail != None {
                    // println!("user --> {:?}",user_detail);
                    return true; 
                }else{
                    // println!("user not exist ");
                    return false; 
                }  
            }else {return false;}
        },
    }
}