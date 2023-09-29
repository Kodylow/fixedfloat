// region:    --- Modules
pub mod error;
mod models;

pub mod routes_fixedfloat;
pub mod routes_static;
pub mod routes_utils;

use tower_cookies::{Cookie, Cookies};
