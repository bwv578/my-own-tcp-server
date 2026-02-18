use std::sync::{Arc, OnceLock, RwLock};
use crate::protocols::http::http::Http;

pub static RW_PROT:OnceLock<Arc<RwLock<Http>>> = OnceLock::new();