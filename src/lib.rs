#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

fn main() {
    let a = unsafe {
        hell()
    };
    
    println!("Rust is here. C++: {}", a);

    std::thread::sleep(std::time::Duration::from_secs(5));
}