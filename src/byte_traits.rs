


pub trait ConvertBytes {
    fn to_ne_bytes(&self) -> Vec<u8>;
    fn size_of(&self) -> usize;
    fn is_bigger(&self, x : f32) -> bool;
}

impl ConvertBytes for f32 {
    fn to_ne_bytes(&self) -> Vec<u8> {
        let y : f32 = self.clone();
        let z = y.to_ne_bytes();
        return Vec::from(z);
    }
    fn size_of(&self) -> usize {
        return 4;
    }
    fn is_bigger(&self, x : f32) -> bool {
        return *self > x;
    }
}

impl ConvertBytes for i16 {
    fn to_ne_bytes(&self) -> Vec<u8> {
        let y : i16 = self.clone();
        let z = y.to_ne_bytes();
        return Vec::from(z);
    }
    fn size_of(&self) -> usize {
        return 2;
    }
    fn is_bigger(&self, x : f32) -> bool {
        return *self as f32 > x;
    }
}

impl ConvertBytes for u16 {
    fn to_ne_bytes(&self) -> Vec<u8> {
        let y : u16 = self.clone();
        let z = y.to_ne_bytes();
        return Vec::from(z);
    }
    fn size_of(&self) -> usize {
        return 2;
    }
    fn is_bigger(&self, x : f32) -> bool {
        return *self as f32 > x;
    }
}