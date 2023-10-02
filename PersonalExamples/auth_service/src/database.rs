pub enum Status {
    Connected,
    Interrupted,
}

pub fn connect_to_db() -> Status {
    Status::Connected
}