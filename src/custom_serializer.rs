use serde::{ser::{Serializer, SerializeStruct}, Serialize};

pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Serialize for Point {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let mut s = serializer.serialize_struct("Point", 2)?;
        s.serialize_field("x", &self.x)?;
        s.serialize_field("y", &self.y)?;
        s.end()
    }
}
