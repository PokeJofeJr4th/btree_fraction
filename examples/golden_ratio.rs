use btree_fraction::{UFrac16, UFrac32, UFrac64, UFrac8};

const GOLDEN: f64 = 1.618_033_988_749_895;

fn main() {
    println!("{}", GOLDEN);
    let gold8 = UFrac8::try_from(GOLDEN).unwrap();
    println!("{gold8} = {gold8:?} = {}", UFrac8::GOLDEN_RATIO);
    let gold16 = UFrac16::try_from(GOLDEN).unwrap();
    println!("{gold16} = {gold16:?} = {}", UFrac16::GOLDEN_RATIO);
    let gold32 = UFrac32::try_from(GOLDEN).unwrap();
    println!("{gold32} = {gold32:?} = {}", UFrac32::GOLDEN_RATIO);
    // The `UFrac64` Golden Ratio is more precise than the `f64` one, so they don't produce the same value 🤯
    let gold64 = UFrac64::try_from(GOLDEN).unwrap();
    println!("{gold64} = {gold64:?} = {}", UFrac64::GOLDEN_RATIO);
}
