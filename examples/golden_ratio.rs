use btree_fraction::{UFrac16, UFrac32, UFrac8};

const GOLDEN: f64 = 1.618_033_988_749_895;

fn main() {
    println!("{}", GOLDEN);
    let gold8 = UFrac8::try_from(GOLDEN).unwrap();
    println!("{gold8} = {gold8:?} = {}", UFrac8::GOLDEN_RATIO);
    let gold16 = UFrac16::try_from(GOLDEN).unwrap();
    println!("{gold16} = {gold16:?} = {}", UFrac16::GOLDEN_RATIO);
    let gold32 = UFrac32::try_from(GOLDEN).unwrap();
    println!("{gold32} = {gold32:?} = {}", UFrac32::GOLDEN_RATIO);
}
