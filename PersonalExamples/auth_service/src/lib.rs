mod database;
mod auth_utils;

fn authenticate (creds: auth_utils::models::Credentials) {
    if let database::Status::Connected = database::connect_to_db() {
        auth_utils::login(creds);
    }

}