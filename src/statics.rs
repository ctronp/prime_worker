static mut PORT_STR: &str = "8080";
static mut PORT_U16: u16 = 8080;
static mut MAX_VALUE_LEN_USIZE: usize = 2000;
// pub static MAX_PAYLOAD: usize = 262_144;

pub fn init_static() {
    static mut INIT: bool = false;
    static mut PORT: String = String::new();
    static mut MAX_VALUE_LEN: String = String::new();

    unsafe {
        if !INIT {
            println!("Initializing Values");
            match std::env::var("PORT") {
                Ok(value) => { if !value.is_empty() { PORT = value } }
                Err(_) => { PORT = PORT_STR.to_string() }
            }
            match std::env::var("MAX_VALUE_LEN") {
                Ok(value) => { if !value.is_empty() { MAX_VALUE_LEN = value } }
                Err(_) => { MAX_VALUE_LEN = MAX_VALUE_LEN_USIZE.to_string() }
            }


            PORT_STR = &PORT[..];
            PORT_U16 = PORT.parse().unwrap()
        }
    }

    if cfg!(debug_assertions) {
        println!("Debug mode on");
        println!("\nVariables:\
        \n  -PORT: {:?}\
        \n  -MAX_VALUE_LEN: {:?}",
                 get_port_u16(),
                 get_max_value_usize()
        )
    }
}

// #[inline]
// pub fn get_port_str() -> &'static str {
//     unsafe { PORT_STR }
// }

#[inline]
pub fn get_port_u16() -> u16 {
    unsafe { PORT_U16 }
}

#[inline]
pub fn get_max_value_usize() -> usize { unsafe { MAX_VALUE_LEN_USIZE } }