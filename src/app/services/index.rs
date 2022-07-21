#![allow(unused)]
use anyhow::{anyhow, bail, Context, Ok, Result};
use aws_sdk_s3::{config, ByteStream, Client, Credentials, Region};
use std::env;
use std::fs::{create_dir_all, File};
use std::io::{BufWriter, Write};
use std::path::Path;
use tokio_stream::StreamExt;

#[path = "../constants/index.rs"]

mod constants;
#[path = "../models/file_upload.rs"]

pub(crate) mod model;



use model::Upload_File;
// file upload
pub async fn file_upload(path: &Path) -> Result<(bool)> {
    let client = get_aws_client(constants::REGION)?;
    let mut a = 1;
    if !path.exists() {
        a = 0;
        bail!("Path {} does not exists", path.display())
    }
    let key = path
        .to_str()
        .ok_or_else(|| anyhow!("Invalid path {path:?}"))?;
    let body = ByteStream::from_path(&path).await?;
    let content_type = mime_guess::from_path(&path)
        .first_or_octet_stream()
        .to_string();
    // BUILD - aws request
    let req = client
        .put_object()
        .bucket(constants::BUCKET_NAME)
        .key(key)
        .body(body)
        .content_type(content_type);
    // EXECUTE
    req.send().await?;
    if a == 1 {
        Ok((true))
    } else {
        Ok((false))
    }
}
pub async fn file_downloaded(file_name: &str) -> Result<bool>{
    // let file_name = "189932.jpg";
    let client = get_aws_client(constants::REGION)?;
    let dir = Path::new("download");
    let mut result = true;
    if !dir.is_dir() {
        // bail!("Path {} is not a directory", dir.display());
        result = false;
    }
    let file_path = dir.join(file_name);
    let parent_dir = file_path
        .parent()
        .ok_or_else(|| anyhow!("Invalid parent dir for {:?}", file_path))?;

    if !parent_dir.exists() {
        create_dir_all(parent_dir)?;
    }

    let req = client.get_object().bucket(constants::BUCKET_NAME).key(file_name);

    // EXECUTE
    let res = req.send().await?;

    // STREAM result to file
    let mut data: ByteStream = res.body;
    let file = File::create(&file_path)?;
    let mut buf_writer = BufWriter::new(file);
    while let Some(bytes) = data.try_next().await? {
        buf_writer.write(&bytes)?;
    }
    buf_writer.flush()?;

    if result {
        Ok((true))
    }else{
        Ok((false))
    }
}
fn get_aws_client(region: &str) -> Result<Client> {
    // get the id/secret from env
    // let key_id = env::var(ENV_CRED_KEY_ID).context("Missing S3_KEY_ID")?;
    // let key_secret = env::var(ENV_CRED_KEY_SECRET).context("Missing S3_KEY_SECRET")?;
    let key_id = (constants::ENV_CRED_KEY_ID).to_string();
    let key_secret = (constants::ENV_CRED_KEY_SECRET).to_string();
    // build the aws cred
    let cred = Credentials::new(key_id, key_secret, None, None, "loaded-from-custom-env");
    // build the aws client
    let region = Region::new(region.to_string());
    let conf_builder = config::Builder::new()
        .region(region)
        .credentials_provider(cred);
    let conf = conf_builder.build();
    // build aws client
    let client = Client::from_conf(conf);
    Ok(client)
}