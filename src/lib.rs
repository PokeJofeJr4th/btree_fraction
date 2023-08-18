#![warn(clippy::nursery, clippy::pedantic)]
#[cfg(test)]
mod tests;
mod ufrac16;
mod ufrac32;
mod ufrac8;

pub use ufrac16::UFrac16;
pub use ufrac32::UFrac32;
pub use ufrac8::UFrac8;
