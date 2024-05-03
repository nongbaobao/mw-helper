trait Modes {
    fn to_string() -> String;
}

pub struct Megawalls {
    pub standard: i32,
    pub faceoff: i32,
}

impl Modes for Megawalls {
    fn to_string() -> String {
        todo!()
    }
}

pub struct UHC {
    pub solo: i32,
    pub teams: i32,
}

impl Modes for UHC {
    fn to_string() -> String {
        todo!()
    }
}