#![warn(clippy::nursery, clippy::pedantic)]
mod ifrac8;
#[cfg(test)]
mod tests;
mod unsigned;

pub use ifrac8::IFrac8;
pub use unsigned::{UFrac16, UFrac32, UFrac64, UFrac8};
