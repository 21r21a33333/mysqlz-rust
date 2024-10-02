pub mod hello_user;
pub use hello_user::*;

pub mod home;
pub use home::*;

pub mod crate_user;
pub use crate_user::*;


pub mod todo;
pub use todo::*;

pub fn logging(path: &str) {
    println!("Path: {}", path);
}