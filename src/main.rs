mod api;

use std::error::Error;

use api::{auth::read_credentials, TumblrClient};

use crate::api::{
    blog::{BlogInfoRequest, BlogInfoResponse, TumblrBlogId},
    requests::TumblrResponse,
};

const CLIENT_CACHE_PATH: &str = "client.json";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let credentials = read_credentials()?;
    let mut tumblr_client = TumblrClient::try_from_file_or_authorize(
        CLIENT_CACHE_PATH.into(),
        credentials,
        reqwest::Client::new(),
    )
    .await?;

    let blog_id = TumblrBlogId::BlogName("the-pink-hacker".to_string());

    let response = tumblr_client
        .request(BlogInfoRequest { blog_id: blog_id }.try_into()?)
        .await?;
    let response = serde_json::from_str::<TumblrResponse<BlogInfoResponse>>(&response)?;
    println!("Response: {:#?}", response);
    tumblr_client.save_to_file(CLIENT_CACHE_PATH.into())?;
    Ok(())
}
