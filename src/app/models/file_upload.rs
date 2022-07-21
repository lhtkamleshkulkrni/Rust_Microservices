#![allow(non_camel_case_types)]

use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize)]
pub struct Upload_File {
    pub file_path: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Download_File {
    pub file_name: String,
}
