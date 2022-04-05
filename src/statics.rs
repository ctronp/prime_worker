#[cfg(test)]
use std::time::Duration;

use tokio::sync::OnceCell;

static mut PORT_STR: &str = "8080";
static mut PORT_U16: u16 = 8080;
static mut MAX_LEN_USIZE: usize = 200;
static mut SECRET_HEADER_STR: &str = "Secret";
static mut SECRET_STR: &str = "SecretStringExample1111000011110000";

pub async fn init_static() {
    static INIT: OnceCell<()> = OnceCell::const_new();
    static mut PORT: String = String::new();
    static mut MAX_LEN: String = String::new();
    static mut SECRET: String = String::new();
    static mut SECRET_HEADER: String = String::new();

    INIT.get_or_init(|| async {
        unsafe {
            log::debug!("Initializing Values");
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
            match std::env::var("MAX_LEN") {
                Ok(value) => {
                    if !value.is_empty() {
                        MAX_LEN = value
                    } else {
                        MAX_LEN = MAX_LEN_USIZE.to_string();
                    }
                }
                Err(_) => MAX_LEN = MAX_LEN_USIZE.to_string(),
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
            match std::env::var("SECRET_HEADER") {
                Ok(value) => {
                    if !value.is_empty() {
                        SECRET_HEADER = value;
                    } else {
                        SECRET_HEADER = SECRET_HEADER_STR.to_string();
                    }
                }
                Err(_) => SECRET = SECRET_HEADER_STR.to_string(),
            }

            SECRET_HEADER_STR = &SECRET_HEADER;
            SECRET_STR = &SECRET;
            PORT_STR = &PORT[..];
            PORT_U16 = PORT.parse().unwrap();
        }

        if cfg!(debug_assertions) {
            log::debug!("Debug mode on");
        } else {
            log::debug!("Debug mode off");
        }
        log::info!(
            "\nVariables:\
        \n  -PORT: {:?}\
        \n  -MAX_LEN: {:?}",
            get_port_u16(),
            get_max_len_usize()
        );
    })
    .await;
}

#[inline]
pub fn get_port_u16() -> u16 {
    unsafe { PORT_U16 }
}

#[inline]
pub fn get_max_len_usize() -> usize {
    unsafe { MAX_LEN_USIZE }
}

#[inline]
pub fn get_secret() -> &'static str {
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
