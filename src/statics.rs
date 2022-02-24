use std::env::VarError;
use once_cell::sync::Lazy;

static mut PORT_STR: &str = "8080";
static mut PORT_USIZE: u16 = 8080;
pub static MAX_PAYLOAD: usize = 262_144;

#[inline]
pub fn init_static() {
    static mut INIT: bool = false;
    static mut PORT: String = String::new();
    unsafe {
        if !INIT {
            println!("Initializing Values");
            match std::env::var("PORT") {
                Ok(value) => { if value != "" { PORT = value } }
                Err(_) => { PORT = PORT_STR.to_string() }
            }


            PORT_STR = &PORT[..];
            PORT_USIZE = PORT.parse().unwrap()
        }
    }
}

#[inline]
pub fn get_port_str() -> &'static str {
    unsafe { PORT_STR }
}

#[inline]
pub fn get_port_usize() -> u16 {
    unsafe { PORT_USIZE }
}