use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    display_name: String,
    email: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccessToken {
    pub token: String,
}
