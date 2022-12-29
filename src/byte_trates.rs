// use std::any::TypeId;




// pub fn from_bytes<T>(bytes : Vec<u8>) -> T {
//     if TypeId::of::<T>() == TypeId::of::<f32>() {
//         let x : [u8; 4] = [0; 4];
//         for i in 0..bytes.len() {
//             x[i] = bytes[i];
//         }
//         return f32::from_ne_bytes(x); //.try_into().expect("FAIL");
//     } else if TypeId::of::<T>() == TypeId::of::<i16>() {
//         let x : [u8; 2] = [0; 2];
//         for i in 0..bytes.len() {
//             x[i] = bytes[i];
//         }
//         return i16::from_ne_bytes(x); 
//     } else {
//         let x : [u8; 2] = [0; 2];
//         for i in 0..bytes.len() {
//             x[i] = bytes[i];
//         }
//         return u16::from_ne_bytes(x); 
//     }
// }


// impl From<f32> for f32 {
//     fn from(f : f32) -> 
// }



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
        let x = self.to_ne_bytes();
        println!("YO2");
        return Vec::from(x);
    }
}

impl ConvertBytes for u16 {
    fn to_ne_bytes(&self) -> Vec<u8> {
        println!("YO3");
        let x = self.to_ne_bytes();
        // return &x[0..x.len()];
        return Vec::from(x);
    }
}





// pub trait FromBytes {
//     pub fn new_from_bytes<U>(&self) -> U;
// }

// impl FromBytes for Vec<f32> {
//     pub fn new_from_bytes<U>(&self) -> U {

//     }
// }