use crate::valclient_rs::errors::ValClientError;
use crate::valclient_rs::UserAuthentication;

pub struct Auth {
    pub username: String,
    pub password: String,
}

impl Auth {
    pub fn new(username: String, password: String) -> Self {
        Self { username, password }
    }

    pub fn none() -> Self {
        Self {
            username: "".to_string(),
            password: "".to_string(),
        }
    }

    pub fn authenticate(&self) -> Result<UserAuthentication, ValClientError> {
        // todo: finish authentication with usernames
        Err(ValClientError::new(
            "Authentication failed",
            "Username authentication is not supported yet",
        ))
    }
}
