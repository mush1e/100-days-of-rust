

mod database {
    pub enum Status {
        Connected,
        Interrupted,
    }
    
    pub fn connect_to_db() -> Status {
        Status::Connected
    }
}



mod auth_utils {
    pub fn login (creds : models::Credentials) {
        fetch_user()
    } 
    
    fn logout (creds : models::Credentials) {
        // implement logout
    }

    fn fetch_user() {

    }

    pub mod models {
        pub struct Credentials {
            username: String,
            password: String,
        }
    }
}

fn authenticate (creds: auth_utils::models::Credentials) {
    if let database::Status::Connected = database::connect_to_db() {
        auth_utils::login(creds);
    }

}