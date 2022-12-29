

fn u32_to_binary(mut i : u32) -> [u8; 32] {
    let mut res : [u8; 32] = [0; 32];
    let mut x : u32 = 31;
    while x >= 0 {
        if 2_u32.pow(x) <= i {
            res[(31-x) as usize] = 1;
            i -= 2_u32.pow(x);
        }
        if x == 0 {
            break;
        }
        x -= 1;
    }
    return res;
}



fn main()
{
    let f = 12.5f32;    
    let x = f.to_ne_bytes();
    let y = u32_to_binary(f.to_bits());
    print!("y: ");
    for i in 0..32 {
        print!("{}", y[i])
    }

    /* MUST CONSIDER IF PLATFORM IS BIG OR SMALL ENDIAN!! */
    let mut z : [u8; 4] = [0; 4];
    let mut j = y.len()-1;
    for i in 0..4 {
        let mut s = 0;
        for p in 0..8 {
            if y[j] == 1 {
                s += 2_u8.pow(p);
            }
            if j == 0 {
                break;
            }
            j -= 1;
        }
        z[i] = s;
    }
    
    println!("x");
    for n in 0..4 {
        println!("{}", x[n]);
    }
    println!("z");
    for n in 0..4 {
        println!("{}", z[n]);
    }
    // println!("EQUAL!");
    
}