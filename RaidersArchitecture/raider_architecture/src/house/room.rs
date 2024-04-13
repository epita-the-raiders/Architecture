#[derive(Debug)]
pub struct Room {
    pub roomType: String,
    pub length: u32,
    pub width: u32,
}

#[derive(Debug)]
pub struct House {
    pub rooms: Vec<Room>,
}