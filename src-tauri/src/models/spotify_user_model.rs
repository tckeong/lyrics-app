use serde::{Deserialize, Serialize};
use super::{Image, ExternalUrls};

#[derive(Serialize, Deserialize, Debug)]
struct Followers {
    href: Option<String>,
    total: u32,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub display_name: Option<String>,
    external_urls: ExternalUrls,
    #[serde(rename = "followers")]
    _followers: Followers,
    href: String,
    id: String,
    #[serde(rename = "images")]
    _images: Vec<Image>,
    #[serde(rename = "type")]
    _user_type: String,
    uri: String,
}