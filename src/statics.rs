#[cfg(test)]
use std::time::Duration;

use tokio::sync::OnceCell;

static mut PORT_STR: &str = "8080";
static mut PORT_U16: u16 = 8080;
static mut MAX_VALUE_LEN_USIZE: usize = 2000;
static mut SECRET_STR: &str = "Wh$qZCjLi7tZ9be2cy26@L&eGuTp5EBys&mJZVY%99iC\
    Nqd42@rM2!ijLnNJ4aA4oHr5VXD!bD!jp#MFgrQf3wZ@Ac8q$ThQyhSf";

pub async fn init_static() {
    static INIT: OnceCell<()> = OnceCell::const_new();
    static mut PORT: String = String::new();
    static mut MAX_VALUE_LEN: String = String::new();
    static mut SECRET: String = String::new();

    INIT.get_or_init(|| async {
        unsafe {
            println!("Initializing Values");
            match std::env::var("PORT") {
                Ok(value) => {
                    if !value.is_empty() {
                        PORT = value
                    } else {
                        PORT = String::from(PORT_STR);
                    }
                }
                Err(_) => PORT = PORT_STR.to_string(),
            }
            match std::env::var("MAX_VALUE_LEN") {
                Ok(value) => {
                    if !value.is_empty() {
                        MAX_VALUE_LEN = value
                    } else {
                        MAX_VALUE_LEN = MAX_VALUE_LEN_USIZE.to_string();
                    }
                }
                Err(_) => MAX_VALUE_LEN = MAX_VALUE_LEN_USIZE.to_string(),
            }
            match std::env::var("SECRET") {
                Ok(value) => {
                    if !value.is_empty() {
                        SECRET = value
                    } else {
                        SECRET = SECRET_STR.to_string();
                    }
                }
                Err(_) => SECRET = SECRET_STR.to_string(),
            }

            PORT_STR = &PORT[..];
            PORT_U16 = PORT.parse().unwrap();
        }
        if cfg!(debug_assertions) {
            println!("Debug mode on");
            println!(
                "\nVariables:\
        \n  -PORT: {:?}\
        \n  -MAX_VALUE_LEN: {:?}",
                get_port_u16(),
                get_max_value_usize()
            )
        }
    })
        .await;
}

#[inline]
pub fn get_port_u16() -> u16 {
    unsafe { PORT_U16 }
}

#[inline]
pub fn get_max_value_usize() -> usize {
    unsafe { MAX_VALUE_LEN_USIZE }
}

#[inline]
fn get_secret() -> &'static str {
    unsafe { SECRET_STR }
}

#[inline]
pub fn check_header(value: &str) -> bool {
    get_secret().eq(value)
}

#[cfg(test)]
pub async fn debug_initialize() {
    static INIT: OnceCell<()> = OnceCell::const_new();
    INIT.get_or_init(|| async {
        std::thread::spawn(crate::main);
        tokio::time::sleep(Duration::from_secs(10)).await;
    })
        .await;
}
