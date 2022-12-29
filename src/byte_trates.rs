


pub trait ConvertBytes {
    fn to_ne_bytes(&self) -> Vec<u8>;
}

impl ConvertBytes for f32 {
    fn to_ne_bytes(&self) -> Vec<u8> {
        let x : Vec<u8> = Vec::new();
        let y : f32 = self.clone();
        let z = y.to_ne_bytes();
        return Vec::from(z);
    }
}

impl ConvertBytes for i16 {
    fn to_ne_bytes(&self) -> Vec<u8> {
        let x : Vec<u8> = Vec::new();
        let y : i16 = self.clone();
        let z = y.to_ne_bytes();
        return Vec::from(z);
    }
}

impl ConvertBytes for u16 {
    fn to_ne_bytes(&self) -> Vec<u8> {
        let x : Vec<u8> = Vec::new();
        let y : u16 = self.clone();
        let z = y.to_ne_bytes();
        return Vec::from(z);
    }
}