#![warn(clippy::nursery, clippy::pedantic)]
#[cfg(test)]
mod tests;
mod ufrac16;
mod ufrac8;

pub use ufrac16::UFrac16;
pub use ufrac8::UFrac8;
