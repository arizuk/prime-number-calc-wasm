#[no_mangle]
pub extern "C" fn get_prime(start: u32) -> u32 {
    let mut i = start + 1;
    loop {
        if is_prime(i) {
            return i;
        }
        i = i + 1;
    }
}

fn is_prime(v: u32) -> bool {
    match v {
        0 => false,
        1 => true,
        2 => true,
        _ => {
            for i in 2..(v - 1) {
                if v % i == 0 {
                    return false;
                }
            }
            return true;
        }
    }
}
