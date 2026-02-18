use once_cell::sync::Lazy;
use std::env;

pub static GEMINI_API: Lazy<String> =
    Lazy::new(|| env::var("GEMINI_API").expect("GEMINI_API environment variable not set"));

pub static USERNAME: Lazy<String> =
    Lazy::new(|| env::var("USERNAME").expect("USERNAME environment variable not set"));

pub static PASSWORD: Lazy<String> =
    Lazy::new(|| env::var("PASSWORD").expect("PASSWORD environment variable not set"));

pub static SERVER: Lazy<String> =
    Lazy::new(|| env::var("SERVER").expect("SERVER environment variable not set"));
