#![warn(clippy::nursery, clippy::pedantic)]
mod ifrac8;
#[cfg(test)]
mod tests;
mod unsigned;

pub use ifrac8::IFrac8;
pub use unsigned::UFrac16;
pub use unsigned::UFrac32;
pub use unsigned::UFrac8;
