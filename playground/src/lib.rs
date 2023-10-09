mod auth_utils;
mod database;
use auth_utils::models::Credentials;
use database::Status;

pub fn autherticate(creds: Credentials) {
    if let Status::Connected = database::connect_to_database() {
        auth_utils::login(creds);
    }
}
